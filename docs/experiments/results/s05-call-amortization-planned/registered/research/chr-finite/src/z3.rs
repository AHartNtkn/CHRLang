//! Scoped incremental Boolean enumeration using the installed Z3 C API.
//! All AST/solver/model references are owned explicitly; query drop pops exclusions.
use crate::{cnf::Cnf, model::QueryDomains};
use std::{
    ffi::{CStr, c_char, c_uint, c_void},
    ptr,
};
type Handle = *mut c_void;
#[link(name = "z3")]
unsafe extern "C" {
    fn Z3_mk_config() -> Handle;
    fn Z3_del_config(c: Handle);
    fn Z3_mk_context_rc(c: Handle) -> Handle;
    fn Z3_del_context(c: Handle);
    fn Z3_mk_solver(c: Handle) -> Handle;
    fn Z3_solver_inc_ref(c: Handle, s: Handle);
    fn Z3_solver_dec_ref(c: Handle, s: Handle);
    fn Z3_mk_bool_sort(c: Handle) -> Handle;
    fn Z3_sort_to_ast(c: Handle, s: Handle) -> Handle;
    fn Z3_mk_int_symbol(c: Handle, i: i32) -> Handle;
    fn Z3_mk_const(c: Handle, s: Handle, t: Handle) -> Handle;
    fn Z3_inc_ref(c: Handle, a: Handle);
    fn Z3_dec_ref(c: Handle, a: Handle);
    fn Z3_mk_not(c: Handle, a: Handle) -> Handle;
    fn Z3_mk_or(c: Handle, n: c_uint, a: *const Handle) -> Handle;
    fn Z3_solver_assert(c: Handle, s: Handle, a: Handle);
    fn Z3_solver_push(c: Handle, s: Handle);
    fn Z3_solver_pop(c: Handle, s: Handle, n: c_uint);
    fn Z3_solver_check(c: Handle, s: Handle) -> i32;
    fn Z3_solver_get_reason_unknown(c: Handle, s: Handle) -> *const c_char;
    fn Z3_solver_get_model(c: Handle, s: Handle) -> Handle;
    fn Z3_model_inc_ref(c: Handle, m: Handle);
    fn Z3_model_dec_ref(c: Handle, m: Handle);
    fn Z3_model_eval(c: Handle, m: Handle, t: Handle, completion: bool, out: *mut Handle) -> bool;
    fn Z3_get_bool_value(c: Handle, a: Handle) -> i32;
    fn Z3_get_version(
        major: *mut c_uint,
        minor: *mut c_uint,
        build: *mut c_uint,
        revision: *mut c_uint,
    );
}
pub fn version() -> [u32; 4] {
    let mut v = [0; 4];
    unsafe { Z3_get_version(&mut v[0], &mut v[1], &mut v[2], &mut v[3]) };
    v
}
pub struct Prepared {
    context: Handle,
    solver: Handle,
    atoms: Vec<Handle>,
}
impl Prepared {
    pub fn new(cnf: &Cnf) -> Result<Self, String> {
        if !cnf.variables.is_multiple_of(3) || cnf.variables > i32::MAX as usize {
            return Err("invalid finite Boolean variable count".into());
        }
        if cnf
            .clauses
            .iter()
            .flatten()
            .any(|x| *x == 0 || x.unsigned_abs() as usize > cnf.variables)
        {
            return Err("literal outside Boolean variables".into());
        }
        // SAFETY: Z3-owned objects use one context; retained references outlive all uses.
        unsafe {
            let config = Z3_mk_config();
            let context = Z3_mk_context_rc(config);
            Z3_del_config(config);
            let solver = Z3_mk_solver(context);
            Z3_solver_inc_ref(context, solver);
            let sort = Z3_mk_bool_sort(context);
            Z3_inc_ref(context, Z3_sort_to_ast(context, sort));
            let atoms = (0..cnf.variables)
                .map(|i| {
                    let a = Z3_mk_const(context, Z3_mk_int_symbol(context, i as i32), sort);
                    Z3_inc_ref(context, a);
                    a
                })
                .collect();
            Z3_dec_ref(context, Z3_sort_to_ast(context, sort));
            let mut prepared = Self {
                context,
                solver,
                atoms,
            };
            for clause in &cnf.clauses {
                prepared.assert_clause(clause)
            }
            Ok(prepared)
        }
    }
    fn assert_clause(&mut self, literals: &[i32]) {
        unsafe {
            let args: Vec<_> = literals
                .iter()
                .map(|lit| {
                    let a = self.atoms[lit.unsigned_abs() as usize - 1];
                    let a = if *lit < 0 {
                        Z3_mk_not(self.context, a)
                    } else {
                        a
                    };
                    Z3_inc_ref(self.context, a);
                    a
                })
                .collect();
            let clause = Z3_mk_or(self.context, args.len() as u32, args.as_ptr());
            Z3_inc_ref(self.context, clause);
            Z3_solver_assert(self.context, self.solver, clause);
            Z3_dec_ref(self.context, clause);
            for a in args {
                Z3_dec_ref(self.context, a)
            }
        }
    }
    pub fn start(&mut self, domains: &QueryDomains) -> Result<Query<'_>, String> {
        if domains.masks.len() * 3 != self.atoms.len() || domains.masks.iter().any(|m| m & !7 != 0)
        {
            return Err("invalid query domains".into());
        }
        unsafe { Z3_solver_push(self.context, self.solver) };
        if domains.inconsistent {
            self.assert_clause(&[])
        }
        for (v, mask) in domains.masks.iter().enumerate() {
            for a in 0..3 {
                if mask & (1 << a) == 0 {
                    self.assert_clause(&[-((1 + 3 * v + a) as i32)])
                }
            }
        }
        Ok(Query {
            prepared: self,
            done: false,
        })
    }
}
impl Drop for Prepared {
    fn drop(&mut self) {
        unsafe {
            for a in &self.atoms {
                Z3_dec_ref(self.context, *a)
            }
            Z3_solver_dec_ref(self.context, self.solver);
            Z3_del_context(self.context)
        }
    }
}
pub struct Query<'a> {
    prepared: &'a mut Prepared,
    done: bool,
}
impl Query<'_> {
    pub fn next_solution(&mut self) -> Result<Option<Vec<u8>>, String> {
        if self.done {
            return Ok(None);
        }
        let p = &mut self.prepared;
        unsafe {
            match Z3_solver_check(p.context, p.solver) {
                -1 => {
                    self.done = true;
                    return Ok(None);
                }
                0 => {
                    self.done = true;
                    return Err(
                        CStr::from_ptr(Z3_solver_get_reason_unknown(p.context, p.solver))
                            .to_string_lossy()
                            .into_owned(),
                    );
                }
                1 => (),
                _ => return Err("invalid solver result".into()),
            }
            let model = Z3_solver_get_model(p.context, p.solver);
            Z3_model_inc_ref(p.context, model);
            let result: Result<Vec<u8>, String> = (|| {
                let mut assignment = vec![];
                for variables in p.atoms.chunks(3) {
                    let mut selected = None;
                    for (a, atom) in variables.iter().enumerate() {
                        let mut value = ptr::null_mut();
                        if !Z3_model_eval(p.context, model, *atom, true, &mut value) {
                            return Err("model evaluation failed".into());
                        }
                        Z3_inc_ref(p.context, value);
                        let truth = Z3_get_bool_value(p.context, value);
                        Z3_dec_ref(p.context, value);
                        match truth {
                            1 => {
                                if selected.replace(a as u8).is_some() {
                                    return Err("non-exclusive finite model".into());
                                }
                            }
                            -1 => (),
                            _ => return Err("incomplete Boolean model".into()),
                        }
                    }
                    assignment.push(selected.ok_or("missing finite value")?)
                }
                Ok(assignment)
            })();
            Z3_model_dec_ref(p.context, model);
            let assignment: Vec<u8> = result?;
            let blocking: Vec<_> = assignment
                .iter()
                .enumerate()
                .map(|(v, a)| -((1 + 3 * v + *a as usize) as i32))
                .collect();
            p.assert_clause(&blocking);
            Ok(Some(assignment))
        }
    }
}
impl Drop for Query<'_> {
    fn drop(&mut self) {
        unsafe { Z3_solver_pop(self.prepared.context, self.prepared.solver, 1) }
    }
}
