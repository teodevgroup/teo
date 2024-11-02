import os
import re

for db in os.listdir('connectors'):
    if db.startswith('.') or db == 'mod.rs':
        continue
    file_content = ''
    with open(f'connectors/{db}/types/mod.rs', 'r') as file:
        file_content = file.read()
        file_content = re.sub(r'set_json', 'json_body', file_content)
        file_content = re.sub(r'.to_request\(\);', '.await.unwrap();', file_content)
        file_content = re.sub(r'test::TestRequest::default\(\)\n\s+\.method\(Method::([a-zA-Z]+)\)\n\s+\.uri\("/([a-zA-Z]+)/([a-zA-Z]+)"\)', lambda matches: f'TestRequest::new(Method::{matches[1]}, "{matches[2]}/{matches[3]}")', file_content)
        file_content = re.sub(r'let res: Value = test::call_and_read_body_json\(&app, req\).await;', 'let res = server().process_test_request(req).await.unwrap().body_as_json().unwrap();', file_content)
        file_content = re.sub(r'use teo::server::server::Server;', 'use hyper::Method;\n    use teo::server::{server::Server, test_request::TestRequest};', file_content)
    with open(f'connectors/{db}/types/mod.rs', 'w') as file:
        file.write(file_content)
