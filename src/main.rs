use regex::Regex;
enum Pattern{
    UuidPattern,
    NumericIdPattern,
    AlphanumericIdPattern,
}
impl Pattern{
    pub fn regex(&self)->Regex{
        match self{
            Pattern::UuidPattern=>Regex::new(r"[A-Fa-f0-9]{8}-[A-Fa-f0-9]{4}-[A-Fa-f0-9]{4}-[A-Fa-f0-9]{4}-[A-Fa-f0-9]{12}").unwrap(),
            Pattern::NumericIdPattern=>Regex::new(r"\b\d{3,}\b").unwrap(),
            Pattern::AlphanumericIdPattern=>Regex::new(r"^[a-zA-Z0-9\-]{7,}$").unwrap(),
        }
    }
}
fn replace_ids(url: &str)-> String{
    if url.contains('[')|url.contains(','){
        return String::from("__IDs__");
    }
    let patterns=[
        Pattern::UuidPattern,
        Pattern::NumericIdPattern,
        Pattern::AlphanumericIdPattern,
    ];
    for pattern in &patterns{
        let regex=pattern.regex();
        if regex.is_match(url){
            return String::from("__ID__");
        }
    }
    url.to_string()
}
fn main() {
    let test_cases = vec![
        ("/ping/fa77c3e6-0514-465b-9962-320643a3ac97", "/ping/__ID__"),
        ("/exec/OjJLMK1-", "/exec/__ID__"),
        ("/exec/21Bn-4Dr", "/exec/__ID__"),
        ("/exec/1seRxkot", "/exec/__ID__"),
        ("/store_items2/_doc/01RNX0D9XM", "/store_items2/_doc/__ID__"),
        ("/store_items2/_doc/02DGL9W3WA", "/store_items2/_doc/__ID__"),
        ("/v1/skus/01JS1MJP9M,47U1ZXNUWO,LYM6X3NBJX,V3ZB96DYW,CDQ456GJ7M,3GPCUM6CM1,ZD0JR6R20S,Q1BL7MTX6S,1UBOGJXLNB,K3KQYD85UD,J2Z93H637H,ZU454HY27F,04KFFWO
        P9T,8HNUAQNY,B8GLJ7TORK,AZL63VH2MC", "/v1/skus/__IDs__"),
    ];
    for (first,second) in test_cases{
        let parts: Vec<String>=first.split('/').map(|x| replace_ids(x)).collect();
        let res=parts.join("/");
        assert_eq!(res,*second);
    }
    println!("Success");
}