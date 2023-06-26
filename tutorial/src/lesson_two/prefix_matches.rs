pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    println!("----------------------------");
    println!("input: {:?}", (prefix, request_path));
    let prifixes = prefix.split('/');
    let request_paths = request_path
        .split('/')
        .map(Some)
        .chain(std::iter::once(None));
    // 为何要用 chain 给补上一个 None？
    // 正常 request 的长度应该大于等于 prefix。所以如果 zip 过程中，出现 request_path 先结束的情况，那么一定不符合。
    // 所以使用 Option 对 request_path 切分后的元素进行转换。
    // 并在最后追加一个 None，用于表达 “出现 request_path 先结束的情况” 的场景。
    for (prefix, request_path) in prifixes.zip(request_paths) {
        match request_path {
            Some(request_path) => {
                if prefix != "*" && prefix != request_path {
                    return false;
                }
            }
            None => {
                return false;
            }
        }
    }
    true
}

#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}
