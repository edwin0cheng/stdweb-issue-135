# stdweb-issue-135
stdweb-issue-135

cargo clean
cargo web start

Open developer console:

Expect : 
```
Hello!
```

Actual :
```
Error loading Rust wasm module 'stdweb_issue_135': 
TypeError: Module.STDWEB_PRIVATE is undefined
Stack trace:
__extjs_da1c2f2f9d03cb526906c689597c9b3520a9e5b3@http://localhost:8000/stdweb-issue-135.js:35:17
stdweb_issue_135::main::h382fe404d9b32aed@http://localhost:8000/stdweb-issue-135.js:45019:1
std::rt::lang_start::{{closure}}::hcd94b9f728cae25b@http://localhost:8000/stdweb-issue-135.js:792:1
std::sys_common::backtrace::__rust_begin_short_backtrace::h3eeaf3c07018dcc1@http://localhost:8000/stdweb-issue-135.js:16077:1
main@http://localhost:8000/stdweb-issue-135.js:44915:1
__instantiate@http://localhost:8000/stdweb-issue-135.js:68:9
__initialize/<@http://localhost:8000/stdweb-issue-135.js:74:17
stdweb-issue-135.js:79:17
TypeError: Module.STDWEB_PRIVATE is undefined
[Learn More]
```
