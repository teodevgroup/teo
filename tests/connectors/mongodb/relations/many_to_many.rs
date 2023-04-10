use test_helpers::*;

#[before_all]
#[after_all]
mod test {
    use serial_test::serial;
    use std::sync::Mutex;
    use serde_json::{json};
    use crate::lib::{ExecutionHandle, req};
    use crate::{assert_json, matcher};
    use crate::lib::matcher_functions::one_match;
    use once_cell::sync::Lazy;

    static HANDLE: Lazy<Mutex<ExecutionHandle>> = Lazy::new(|| {
        Mutex::new(ExecutionHandle::new())
    });
    static PORT: i32 = 4020;

    fn before_all() {
        HANDLE.lock().unwrap().execute(file!(), "serve");
    }

    fn after_all() {
        HANDLE.lock().unwrap().exit();
    }

    #[serial]
    #[test]
    fn create_with_nested_create_one() {
        let _create_res = req(PORT, "create", "Artist", json!({
            "create": {
                "name": "Taylor Swift",
                "songs": {
                    "create": {
                        "name": "Love Story"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Artist", json!({
            "include": {
                "songs": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Taylor Swift",
            "songs": [
                {
                    "id": ignore,
                    "name": "Love Story"
                }
            ]
        }))));
    }

    #[serial]
    #[test]
    fn create_with_nested_create_many() {
        let _create_res = req(PORT, "create", "Artist", json!({
            "create": {
                "name": "Taylor Swift",
                "songs": {
                    "createMany": [
                        {
                            "name": "Love Story"
                        },
                        {
                            "name": "Red"
                        }
                    ]
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Taylor Swift",
            "songs": [
                {
                    "id": ignore,
                    "name": "Love Story"
                },
                {
                    "id": ignore,
                    "name": "Red"
                }
            ]
        }))));
    }

    #[serial]
    #[test]
    fn create_with_nested_connect_one() {
        let _create_res = req(PORT, "create", "Artist", json!({
            "create": {
                "name": "Taylor Swift",
                "songs": {
                    "connect": {
                        "name": "Perfect"
                    },
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Taylor Swift",
            "songs": [
                {
                    "id": ignore,
                    "name": "Perfect"
                },
            ]
        }))));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": [
                {
                    "id": ignore,
                    "name": "Perfect"
                },
                {
                    "id": ignore,
                    "name": "Shape of You"
                }
            ]
        }))));
    }

    #[serial]
    #[test]
    fn create_with_nested_connect_more_than_one() {
        let _create_res = req(PORT, "create", "Artist", json!({
            "create": {
                "name": "Taylor Swift",
                "songs": {
                    "connect": [
                        {
                            "name": "Perfect"
                        },
                        {
                            "name": "Maps"
                        }
                    ]
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 3 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Taylor Swift",
            "songs": [
                {
                    "id": ignore,
                    "name": "Maps"
                },
                {
                    "id": ignore,
                    "name": "Perfect"
                },
            ]
        }))));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": [
                {
                    "id": ignore,
                    "name": "Perfect"
                },
                {
                    "id": ignore,
                    "name": "Shape of You"
                }
            ]
        }))));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Maroon 5",
            "songs": [
                {
                    "id": ignore,
                    "name": "Maps"
                },
                {
                    "id": ignore,
                    "name": "Payphone"
                }
            ]
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_create_one() {
        let _update_res = req(PORT, "update", "Artist", json!({
            "where": {
                "name": "Ed Sheeran"
            },
            "update": {
                "songs": {
                    "create": {
                        "name": "Photograph"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": [
                {
                    "id": ignore,
                    "name": "Perfect"
                },
                {
                    "id": ignore,
                    "name": "Photograph"
                },
                {
                    "id": ignore,
                    "name": "Shape of You"
                }
            ]
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_create_many() {
        let _update_res = req(PORT, "update", "Artist", json!({
            "where": {
                "name": "Ed Sheeran"
            },
            "update": {
                "songs": {
                    "createMany": [
                        {
                            "name": "Photograph"
                        },
                        {
                            "name": "Castle on the Hill"
                        }
                    ]
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": [
                {
                    "id": ignore,
                    "name": "Castle on the Hill"
                },
                {
                    "id": ignore,
                    "name": "Perfect"
                },
                {
                    "id": ignore,
                    "name": "Photograph"
                },
                {
                    "id": ignore,
                    "name": "Shape of You"
                }
            ]
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_connect_one() {
        let _update_res = req(PORT, "update", "Artist", json!({
            "where": {
                "name": "Ed Sheeran"
            },
            "update": {
                "songs": {
                    "connect": {
                        "name": "Maps"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": [
                {
                    "id": ignore,
                    "name": "Maps"
                },
                {
                    "id": ignore,
                    "name": "Perfect"
                },
                {
                    "id": ignore,
                    "name": "Shape of You"
                }
            ]
        }))));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Maroon 5",
            "songs": [
                {
                    "id": ignore,
                    "name": "Maps"
                },
                {
                    "id": ignore,
                    "name": "Payphone"
                }
            ]
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_connect_more_than_one() {
        let _update_res = req(PORT, "update", "Artist", json!({
            "where": {
                "name": "Ed Sheeran"
            },
            "update": {
                "songs": {
                    "connect": [
                        {
                            "name": "Maps"
                        },
                        {
                            "name": "Payphone"
                        }
                    ]
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": [
                {
                    "id": ignore,
                    "name": "Maps"
                },
                {
                    "id": ignore,
                    "name": "Payphone"
                },
                {
                    "id": ignore,
                    "name": "Perfect"
                },
                {
                    "id": ignore,
                    "name": "Shape of You"
                }
            ]
        }))));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Maroon 5",
            "songs": [
                {
                    "id": ignore,
                    "name": "Maps"
                },
                {
                    "id": ignore,
                    "name": "Payphone"
                }
            ]
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_set() {
        let _update_res = req(PORT, "update", "Artist", json!({
            "where": {
                "name": "Ed Sheeran"
            },
            "update": {
                "songs": {
                    "set": [
                        {
                            "name": "Maps"
                        },
                        {
                            "name": "Payphone"
                        }
                    ]
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": [
                {
                    "id": ignore,
                    "name": "Maps"
                },
                {
                    "id": ignore,
                    "name": "Payphone"
                },
            ]
        }))));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Maroon 5",
            "songs": [
                {
                    "id": ignore,
                    "name": "Maps"
                },
                {
                    "id": ignore,
                    "name": "Payphone"
                }
            ]
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_disconnect_one() {
        let _update_res = req(PORT, "update", "Artist", json!({
            "where": {
                "name": "Ed Sheeran"
            },
            "update": {
                "songs": {
                    "disconnect": {
                        "name": "Shape of You"
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Artist", json!({
            "include": {
                "songs": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": [
                {
                    "id": ignore,
                    "name": "Perfect"
                },
            ]
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_disconnect_more_than_one() {
        let _update_res = req(PORT, "update", "Artist", json!({
            "where": {
                "name": "Ed Sheeran"
            },
            "update": {
                "songs": {
                    "disconnect": [
                        {
                            "name": "Shape of You"
                        },
                        {
                            "name": "Perfect"
                        }
                    ]
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Artist", json!({
            "include": {
                "songs": true
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": []
        }))));
    }

    #[serial]
    #[test]
    fn update_with_nested_update_one() {
        let _update_res = req(PORT, "update", "Artist", json!({
            "where": {
                "name": "Ed Sheeran"
            },
            "update": {
                "songs": {
                    "update": {
                        "where": {
                            "name": "Shape of You"
                        },
                        "update": {
                            "name": "Shape of You - Radio Edit"
                        }
                    }
                }
            },
        }));
        let find_many_res = req(PORT, "findMany", "Artist", json!({
            "include": {
                "songs": {
                    "orderBy": {
                        "name": "asc"
                    }
                }
            }
        }));
        assert_json!(find_many_res.get("meta").unwrap(), matcher!({ "count": 2 }));
        assert_json!(find_many_res.get("data").unwrap(), matcher!(one_match(matcher!({
            "id": ignore,
            "name": "Ed Sheeran",
            "songs": [
                {
                    "id": ignore,
                    "name": "Perfect"
                },
                {
                    "id": ignore,
                    "name": "Shape of You - Radio Edit"
                },
            ]
        }))));
    }
}
