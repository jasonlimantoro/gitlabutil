use crate::accessor::gitlab as gitlab_accessor;
use crate::manager::gitlab as gitlab_manager;
use crate::modules::merge_request;

pub struct Registry {
    pub merge_request_module: merge_request::Module,
}

impl Registry {
    pub fn new() -> Self {
        let gitlab_http_client = gitlab_accessor::HttpClient::new();
        let gitlab_accessor = gitlab_accessor::Accessor::new(gitlab_http_client);
        let gitlab_manager = gitlab_manager::Manager::new(gitlab_accessor);

        Registry {
            merge_request_module: merge_request::Module::new(gitlab_manager),
        }
    }
}
