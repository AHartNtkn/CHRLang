//! Bounded, backend-independent binary session protocol (only syntax + std).
//!
//! Each frame is u32 little-endian payload byte length followed by that payload.
//! All sequence lengths and UTF-8 string byte lengths are u32 LE; variable IDs
//! are u64 LE. A term is tag 0 then variable ID, or tag 1 then constructor string,
//! argument count, and argument terms. Query: constraint sequence then output
//! sequence; constraint: name string then term sequence; query output: name
//! string then variable ID. Answer: output sequence (name string + term), then
//! residual constraint sequence. Response: tag 0 + Answer, 1 (Failure), or tags
//! 2/3/4 + string (Unsupported/Malformed/Error). Trailing bytes are invalid.
//!
//! MAX_NODES counts every term, constraint and output entry across one payload;
//! root term depth is 1. Limits apply identically to encoding and decoding.
//! Codecs consume/produce payloads, not envelopes. `serve` flushes each response.
//! A complete malformed payload yields Malformed and the session continues.
//! Oversized declared frames stop with InvalidData without reading their payload;
//! truncated headers/payloads stop with UnexpectedEof. Clean EOF is successful.
//! An unencodable backend response yields Error, never logical Failure.
use chr_syntax::{Answer, Constraint, Query, Term, Var};
use std::io::{self, Read, Write};

pub const MAX_FRAME: usize = 1 << 20;
pub const MAX_DEPTH: usize = 512;
pub const MAX_NODES: usize = 100_000;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Response {
    Success(Answer),
    Failure,
    Unsupported(String),
    Malformed(String),
    Error(String),
}
#[derive(Default)]
struct Encoder {
    bytes: Vec<u8>,
    nodes: usize,
}
impl Encoder {
    fn put(&mut self, bytes: &[u8]) -> Result<(), String> {
        if bytes.len() > MAX_FRAME - self.bytes.len() {
            return Err("frame byte limit exceeded".into());
        }
        self.bytes.extend_from_slice(bytes);
        Ok(())
    }
    fn length(&mut self, value: usize) -> Result<(), String> {
        let n = u32::try_from(value).map_err(|_| "length exceeds u32")?;
        self.put(&n.to_le_bytes())
    }
    fn string(&mut self, s: &str) -> Result<(), String> {
        self.length(s.len())?;
        self.put(s.as_bytes())
    }
    fn node(&mut self) -> Result<(), String> {
        self.nodes += 1;
        if self.nodes > MAX_NODES {
            Err("node limit exceeded".into())
        } else {
            Ok(())
        }
    }
    fn term(&mut self, t: &Term, depth: usize) -> Result<(), String> {
        if depth > MAX_DEPTH {
            return Err("term depth limit exceeded".into());
        }
        self.node()?;
        match t {
            Term::Var(Var(id)) => {
                self.put(&[0])?;
                self.put(&id.to_le_bytes())
            }
            Term::App(name, args) => {
                self.put(&[1])?;
                self.string(name)?;
                self.length(args.len())?;
                for arg in args {
                    self.term(arg, depth + 1)?;
                }
                Ok(())
            }
        }
    }
    fn constraints(&mut self, cs: &[Constraint]) -> Result<(), String> {
        self.length(cs.len())?;
        for c in cs {
            self.node()?;
            self.string(&c.name)?;
            self.length(c.args.len())?;
            for t in &c.args {
                self.term(t, 1)?;
            }
        }
        Ok(())
    }
    fn answer(&mut self, a: &Answer) -> Result<(), String> {
        self.length(a.outputs.len())?;
        for (name, t) in &a.outputs {
            self.node()?;
            self.string(name)?;
            self.term(t, 1)?;
        }
        self.constraints(&a.residual)
    }
}
struct Decoder<'a> {
    bytes: &'a [u8],
    position: usize,
    nodes: usize,
}
impl<'a> Decoder<'a> {
    fn new(bytes: &'a [u8]) -> Result<Self, String> {
        if bytes.len() > MAX_FRAME {
            return Err("frame byte limit exceeded".into());
        }
        Ok(Self {
            bytes,
            position: 0,
            nodes: 0,
        })
    }
    fn take(&mut self, n: usize) -> Result<&'a [u8], String> {
        if n > self.bytes.len() - self.position {
            return Err("truncated payload".into());
        }
        let bytes = &self.bytes[self.position..self.position + n];
        self.position += n;
        Ok(bytes)
    }
    fn byte(&mut self) -> Result<u8, String> {
        Ok(self.take(1)?[0])
    }
    fn length(&mut self) -> Result<usize, String> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()) as usize)
    }
    fn count(&mut self) -> Result<usize, String> {
        let n = self.length()?;
        if n > MAX_NODES - self.nodes {
            return Err("node limit exceeded".into());
        }
        Ok(n)
    }
    fn var(&mut self) -> Result<Var, String> {
        Ok(Var(u64::from_le_bytes(self.take(8)?.try_into().unwrap())))
    }
    fn string(&mut self) -> Result<String, String> {
        let n = self.length()?;
        std::str::from_utf8(self.take(n)?)
            .map(String::from)
            .map_err(|_| "invalid UTF-8".into())
    }
    fn node(&mut self) -> Result<(), String> {
        self.nodes += 1;
        if self.nodes > MAX_NODES {
            Err("node limit exceeded".into())
        } else {
            Ok(())
        }
    }
    fn term(&mut self, depth: usize) -> Result<Term, String> {
        if depth > MAX_DEPTH {
            return Err("term depth limit exceeded".into());
        }
        self.node()?;
        match self.byte()? {
            0 => Ok(Term::Var(self.var()?)),
            1 => {
                let name = self.string()?;
                let count = self.count()?;
                let mut args = vec![];
                for _ in 0..count {
                    args.push(self.term(depth + 1)?)
                }
                Ok(Term::App(name, args))
            }
            _ => Err("invalid term tag".into()),
        }
    }
    fn constraints(&mut self) -> Result<Vec<Constraint>, String> {
        let n = self.count()?;
        let mut result = vec![];
        for _ in 0..n {
            self.node()?;
            let name = self.string()?;
            let count = self.count()?;
            let mut args = vec![];
            for _ in 0..count {
                args.push(self.term(1)?)
            }
            result.push(Constraint { name, args });
        }
        Ok(result)
    }
    fn answer(&mut self) -> Result<Answer, String> {
        let n = self.count()?;
        let mut outputs = vec![];
        for _ in 0..n {
            self.node()?;
            outputs.push((self.string()?, self.term(1)?));
        }
        Ok(Answer {
            outputs,
            residual: self.constraints()?,
        })
    }
    fn finish(self) -> Result<(), String> {
        if self.position == self.bytes.len() {
            Ok(())
        } else {
            Err("trailing payload bytes".into())
        }
    }
}
pub fn encode_query(q: &Query) -> Result<Vec<u8>, String> {
    let mut e = Encoder::default();
    e.constraints(&q.constraints)?;
    e.length(q.outputs.len())?;
    for (name, Var(id)) in &q.outputs {
        e.node()?;
        e.string(name)?;
        e.put(&id.to_le_bytes())?;
    }
    Ok(e.bytes)
}
pub fn decode_query(bytes: &[u8]) -> Result<Query, String> {
    let mut d = Decoder::new(bytes)?;
    let constraints = d.constraints()?;
    let n = d.count()?;
    let mut outputs = vec![];
    for _ in 0..n {
        d.node()?;
        outputs.push((d.string()?, d.var()?));
    }
    d.finish()?;
    Ok(Query {
        constraints,
        outputs,
    })
}
pub fn encode_response(r: &Response) -> Result<Vec<u8>, String> {
    let mut e = Encoder::default();
    match r {
        Response::Success(a) => {
            e.put(&[0])?;
            e.answer(a)?;
        }
        Response::Failure => e.put(&[1])?,
        Response::Unsupported(s) => {
            e.put(&[2])?;
            e.string(s)?;
        }
        Response::Malformed(s) => {
            e.put(&[3])?;
            e.string(s)?;
        }
        Response::Error(s) => {
            e.put(&[4])?;
            e.string(s)?;
        }
    }
    Ok(e.bytes)
}
pub fn decode_response(bytes: &[u8]) -> Result<Response, String> {
    let mut d = Decoder::new(bytes)?;
    let r = match d.byte()? {
        0 => Response::Success(d.answer()?),
        1 => Response::Failure,
        2 => Response::Unsupported(d.string()?),
        3 => Response::Malformed(d.string()?),
        4 => Response::Error(d.string()?),
        _ => return Err("invalid response tag".into()),
    };
    d.finish()?;
    Ok(r)
}
pub fn serve<R: Read, W: Write, F: FnMut(Query) -> Response>(
    mut reader: R,
    mut writer: W,
    mut execute: F,
) -> io::Result<()> {
    loop {
        let mut header = [0; 4];
        loop {
            match reader.read(&mut header[..1]) {
                Ok(0) => return Ok(()),
                Ok(_) => break,
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            }
        }
        reader.read_exact(&mut header[1..])?;
        let len = u32::from_le_bytes(header) as usize;
        if len > MAX_FRAME {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "frame byte limit exceeded",
            ));
        }
        let mut bytes = vec![0; len];
        reader.read_exact(&mut bytes)?;
        let response = match decode_query(&bytes) {
            Ok(q) => execute(q),
            Err(message) => Response::Malformed(message),
        };
        let payload = match encode_response(&response) {
            Ok(bytes) => bytes,
            Err(message) => {
                encode_response(&Response::Error(format!("response encoding: {message}")))
                    .map_err(io::Error::other)?
            }
        };
        writer.write_all(&(payload.len() as u32).to_le_bytes())?;
        writer.write_all(&payload)?;
        writer.flush()?;
    }
}
