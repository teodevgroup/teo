use mongodb::options::ClientOptions;
use tokio::test;
use teo::connectors::mongodb::MongoDBConnectorHelpers;
use teo::core::graph::Graph;
use teo::core::pipeline::Pipeline;


async fn make_graph() -> &'static Graph {

    let options = ClientOptions::parse("mongodb://localhost:27017/teotestserver").await.unwrap();

    let graph = Box::leak(Box::new(Graph::new(|g| {
        g.mongodb(options.clone());

        g.model("MyUser", |m| {
            m.field("id", |f| {
                f.required().primary().readonly().object_id().assigned_by_database();
            });
            m.field("name", |f| {
                f.required().string().default("Bson");
            });
            m.field("age", |f| {
                f.required().u8().default(18u8);
            });
            m.permissions(|p| {
                p.can_read(|p| {
                    p.any(vec![
                        |p: &mut Pipeline| { p.is_this_object(); },
                        |p: &mut Pipeline| { p.is_instance_of("Admin"); }
                    ]);
                });
            });
        });

        g.model("AuthCode", |m| {
            m.localized_name("短信验证码");
            m.description("用于用户登录或者修改手机号码的短信验证码。");
            m.field("id", |f| {
                f.primary().required().readonly().object_id().assigned_by_database();
            });
            m.field("phoneNo", |f| {
                f.localized_name("电话号码");
                f.description("接收验证码的电话号码，必填。");
                f.unique().required().string().on_set(|p| {
                    p.regex_match(r"^1\d{10}$");
                });
            });
            m.field("code", |f| {
                f.localized_name("验证码");
                f.description("是一个4位数的数字。");
                f.required().internal().string().on_save(|p| {
                    p.random_digits(4);
                });
            });
            m.field("createdAt", |f| {
                f.required().readonly().datetime().on_save(|p| {
                    p.if_p(|p| { p.is_null(); }).then_p(|p| { p.now(); });
                });
            });
            m.field("updatedAt", |f| {
                f.required().readonly().datetime().on_save(|p| {
                    p.now();
                });
            });
        });

        g.r#enum("Sex", vec!["MALE", "FEMALE"]);

        g.model("User", |m| {
            m.localized_name("用户");
            m.description("在前端平台登录的用户。");
            m.identity();
            m.field("id", |f| {
                f.primary().required().readonly().object_id().assigned_by_database();
            });
            m.field("authCode", |f| {
                f.localized_name("短信验证码");
                f.description("用户必须使用短信验证码来登录系统或修改个人的手机号码。");
                f.temp().optional().string();
            });
            m.field("phoneNo", |f| {
                f.localized_name("电话号码");
                f.description("用户的电话号码，必填，用作登录身份。");
                f.unique().required().string().auth_identity().on_set(|p| {
                    p.regex_match(r"^1\d{10}$");
                    // p.validate_p(|p| {
                    //     p.object_value("authCode").is_equal_p(|p| {
                    //         p.find_unique("AuthCode", {"": ""})
                    //     });
                    // })
                });
            });
            m.field("name", |f| {
                f.localized_name("用户的显示的名字");
                f.description("新用户如果没有传自己的名字，则会默认成为“用户159****8899”这样的格式。");
                f.required().string().default(|p: &mut Pipeline| {
                    p.object_value("phoneNo").regex_replace(r"(.).{3}$", "****").str_prepend("用户");
                });
            });
            m.field("sex", |f| {
                f.localized_name("用户的性别");
                f.description("默认为空，只允许修改一次。");
                f.optional().r#enum("Sex").write_once();
            });
            m.field("posts", |f| {
                f.localized_name("文章列表");
                f.description("用户发表的文章列表。");
                f.vec(|f| {
                    f.object("Post");
                }).linked_by("author");
            });
            m.field("createdAt", |f| {
                f.required().readonly().datetime().on_save(|p| {
                    p.if_p(|p| { p.is_null(); }).then_p(|p| { p.now(); });
                });
            });
            m.field("updatedAt", |f| {
                f.required().readonly().datetime().on_save(|p| {
                    p.now();
                });
            });
        });

        g.model("Post", |m| {
            m.localized_name("文章");
            m.description("用户所写的文章。");
            m.field("id", |f| {
                f.primary().required().readonly().object_id().assigned_by_database();
            });
            m.field("title", |f| {
                f.localized_name("文章标题");
                f.description("文章的标题，必填。");
                f.required().string();
            });
            m.field("content", |f| {
                f.localized_name("文章内容");
                f.description("文章的正文内容，必填。");
                f.required().string();
            });
            m.field("link", |f| {
                f.localized_name("文章原文链接");
                f.description("文章的原文链接，必填。我们只做用户自己转载，不允许成为第一发布平台。");
                f.required().string();
            });
            m.field("author", |f| {
                f.required().object("User").link_to().assign_identity();
            });
        });

        g.model("Admin", |m| {
            m.localized_name("管理员");
            m.description("在管理平台登录的公司内部的管理员。");
            m.field("id", |f| {
                f.primary().required().readonly().object_id().assigned_by_database();
            });
            m.field("email", |f| {
                f.unique().required().string().auth_identity().on_save(|p| {
                    p.email();
                });
            });
            m.field("password", |f| {
               f.writeonly().required().string().auth_by(|p: &mut Pipeline| {
                   p.bcrypt_verify(|p: &mut Pipeline| {
                       p.object_value("password");
                   });
               }).on_set(|p| {
                   p.length_between(8, 16).secure_password().bcrypt_salt();
               });
            });
            m.field("name", |f| {
                f.required().string();
            });
            m.field("activated", |f| {
                f.required().bool().default(true);
            });
            m.field("createdAt", |f| {
                f.required().readonly().datetime().on_save(|p| {
                    p.if_p(|p| { p.is_null(); }).then_p(|p| { p.now(); });
                });
            });
            m.field("updatedAt", |f| {
                f.required().readonly().datetime().on_save(|p| {
                    p.now();
                });
            });
            m.identity();
        });

        g.jwt_secret("my secret");

    }).await));

    graph
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let graph = make_graph().await;
    graph.start_server(5000).await
}
