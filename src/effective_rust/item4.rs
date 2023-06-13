use std::error::Error;

#[derive(Debug)]
pub enum MyError {
    Io(std::io::Error),
    Utf8(std::string::FromUtf8Error),
    General(String),
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::Io(e) => write!(f, "IO error: {}", e),
            MyError::Utf8(e) => write!(f, "UTF-8 error: {}", e),
            MyError::General(s) => write!(f, "General error: {}", s),
        }
    }
}

// 通过实现 source 方法，我们可以在错误链中表示出当前错误的上一级错误。这对于调试和错误处理非常有用，可以帮助我们了解错误发生的原因，并且在错误传播时保留错误的完整上下文信息。
// 自定义的 Error 类型如果实现了 source 方法，可以让我们建立错误链，将底层的原因逐级传递。这样，在处理错误时，我们可以遍历错误链，获取完整的错误信息，以便更好地理解错误的发生和处理。
// 我们可以通过调用 source 方法获取底层的 std::io::Error，从而了解文件操作失败的具体原因。这种错误链的构建能够提供更丰富的错误上下文，并帮助我们更好地进行错误处理和调试。
// 通过实现 source 方法，我们可以使用标准库中提供的错误类型并与自定义错误类型一起工作，形成错误链，方便错误处理和调试。
// 如果文件打开或读取出错，我们将获得一个包含底层错误的 CustomError。通过打印错误信息和错误链，我们可以清楚地看到文件操作的失败原因，并追溯到底层的 std::io::Error。这样，我们可以更好地了解错误的来源，进行错误处理和调试。
impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MyError::Io(e) => Some(e),
            MyError::Utf8(e) => Some(e),
            MyError::General(_) => None,
        }
    }
}

pub fn first_line(filename: &str) -> Result<String, MyError> {
    let file = std::fs::File::open(filename).map_err(MyError::Io)?;

    let mut buf = vec![];

    let result = String::from_utf8(buf).map_err(MyError::Utf8)?;

    Ok(result)
}
