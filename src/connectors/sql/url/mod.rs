pub(crate) mod url_utils {
    use std::path::PathBuf;
    use path_absolutize::Absolutize;
    use url::Url;
    use whoami::Platform;
    use crate::connectors::sql::schema::dialect::SQLDialect;

    pub(crate) fn remove_scheme(url: &str) -> &str {
        if url.starts_with("file://") {
            &url[7..]
        } else if url.starts_with("sqlite://") {
            &url[9..]
        } else if url.starts_with("file:") {
            &url[5..]
        } else if url.starts_with("sqlite:") {
            &url[7..]
        } else if url.starts_with("mysql://") {
            &url[8..]
        } else if url.starts_with("postgres://") {
            &url[11..]
        } else if url.starts_with("mssql://") {
            &url[8..]
        } else {
            url
        }
    }

    pub(crate) fn is_memory_url(url: &str) -> bool {
        url == ":memory:"
    }

    pub(crate) fn absolutized(url: &str) -> PathBuf {
        let path = PathBuf::from(url);
        path.absolutize().unwrap().into_owned()
    }

    pub(crate) fn normalized_url(dialect: SQLDialect, url: &str) -> Url {
        let mut url = Url::parse(url).unwrap();
        match dialect {
            SQLDialect::MySQL => {
                if url.username() == "" {
                    url.set_username("root").unwrap();
                    if url.password().is_none() {
                        url.set_password(Some("")).unwrap();
                    }
                }
            }
            SQLDialect::PostgreSQL => {
                if url.username() == "" {
                    if whoami::platform() == Platform::MacOS {
                        let username = whoami::username();
                        url.set_username(username.as_str()).unwrap();
                    } else {
                        url.set_username("postgres").unwrap();
                    }
                    if url.password().is_none() {
                        url.set_password(Some("")).unwrap();
                    }
                }
            }
            _ => (),
        }
        url
    }

    pub(crate) fn remove_db_path(dialect: SQLDialect, url: &Url) -> Url {
        let mut retval = url.clone();
        if dialect == SQLDialect::PostgreSQL {
            retval.set_path("/postgres");
        } else {
            retval.set_path("/");
        }
        retval
    }
}
