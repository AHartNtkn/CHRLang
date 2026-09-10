"""Independent decoder for the registered binary answer grammar."""
def decode(data, predicates, atoms):
    at = 0
    def byte():
        nonlocal at
        assert at < len(data)
        n=data[at];at+=1;return n
    def word():
        nonlocal at
        assert at+4<=len(data)
        n=int.from_bytes(data[at:at+4],'little');at+=4;return n
    def terms():
        out=[]
        while True:
            tag=byte()
            if tag==0:return out
            assert tag in [1,2]
            n=word();out.append(n if tag==1 else atoms[n])
    answers=[]
    while at<len(data):
        assert byte()==4
        outputs=terms();residual=[]
        while True:
            tag=byte()
            if tag==0:break
            assert tag==3
            pred=predicates[word()];residual.append([pred,terms()])
        answers.append([outputs,residual])
    return answers

def records(data):
    out={};at=0
    while at<len(data):
        end=data.index(b'\n',at);header=data[at:end].split();assert header[0]==b'RESULT'
        index,size=map(int,header[1:]);assert index not in out
        at=end+1;assert at+size<=len(data);out[index]=data[at:at+size];at+=size
    return out

if __name__=='__main__':
    # Answer [unknown7, atom a], with two identical p(a) occurrences.
    data=bytes.fromhex('04 01 07000000 02 00000000 00 03 00000000 02 00000000 00 03 00000000 02 00000000 00 00')
    assert decode(data,['p'],['a'])==[[[7,'a'],[['p',['a']],['p',['a']]]]]
    for broken in [data[:-1],b'\x09',b'\x04\x02\x00']:
        try:decode(broken,['p'],['a'])
        except (AssertionError,IndexError):pass
        else:raise AssertionError('malformed wire accepted')
    print('Independent format vector and truncation checks pass.')
