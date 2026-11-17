pkg {
    name = "hello",
    version = "2.12.3",
    description = "GNU Hello, A simple greeting program",

    artifacts = {
        {
            urls = {
                "https://deb.debian.org/debian/pool/main/h/hello/hello_2.12.3-1_amd64.deb",
                "https://ftp.debian.org/debian/pool/main/h/hello/hello_2.12.3-1_amd64.deb",
            },

            sha256 = "fcd5f5b2169b376ce84d2b541e1e1c2bf5b855eb5e322dd623a88395e2e7458e",
        },
    },

    runtime_deps = { "glibc" },
}
