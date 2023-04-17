 async fn load_config_from_parser(&mut self, parser: &ASTParser) {
        // connector
        let connector_ref = parser.connector.unwrap();
        let source = parser.get_source(connector_ref.0);
        let connector_declaration = source.get_connector(connector_ref.1);
        let url = connector_declaration.url.as_ref().unwrap();
        let connector: Arc<dyn Connector> = match connector_declaration.provider.unwrap() {
            DatabaseName::MySQL => {
                #[cfg(feature = "data-source-mysql")]
                Arc::new(SQLConnector::new(SQLDialect::MySQL, url, false).await)
            },
            DatabaseName::PostgreSQL => {
                #[cfg(feature = "data-source-postgres")]
                Arc::new(SQLConnector::new(SQLDialect::PostgreSQL, url, false).await)
            },
            #[cfg(feature = "data-source-sqlite")]
            DatabaseName::SQLite => {
                #[cfg(feature = "data-source-sqlite")]
                Arc::new(SQLConnector::new(SQLDialect::SQLite, url, false).await)
            },
            DatabaseName::MongoDB => {
                #[cfg(feature = "data-source-mongodb")]
                Arc::new(MongoDBConnector::new(url.clone()).await)
            },
        };
        self.connector = Some(connector.clone());
    }
