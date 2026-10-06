use tauri::Url;

pub const CHAT_URL: &str = "https://chat.deepseek.com/";

fn is_deepseek_host(host: &str) -> bool {
    host == "deepseek.com" || host.ends_with(".deepseek.com")
}

/// Only official pages and specific login providers may stay inside the app.
/// This applies to top-level navigation, not the website's asset/API requests.
pub fn may_embed(url: &Url) -> bool {
    if url.as_str() == "about:blank" {
        return true;
    }

    // The website may open locally generated PDFs or downloads in a new view.
    if let Some(origin) = url.as_str().strip_prefix("blob:") {
        return Url::parse(origin).is_ok_and(|origin| is_official(&origin));
    }

    if !is_secure_web_url(url) {
        return false;
    }

    url.host_str().is_some_and(|host| {
        is_deepseek_host(host)
            || matches!(
                host,
                "appleid.apple.com" | "accounts.google.com" | "open.weixin.qq.com"
            )
    })
}

pub fn is_official(url: &Url) -> bool {
    is_secure_web_url(url) && url.host_str().is_some_and(is_deepseek_host)
}

fn is_secure_web_url(url: &Url) -> bool {
    url.scheme() == "https"
        && url.username().is_empty()
        && url.password().is_none()
        && url.port_or_known_default() == Some(443)
}

pub fn may_open_in_browser(url: &Url) -> bool {
    matches!(url.scheme(), "https" | "http" | "mailto")
        && url.username().is_empty()
        && url.password().is_none()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn official_chat_and_login_redirects_stay_in_the_app() {
        for value in [
            CHAT_URL,
            "https://chat.deepseek.com/a/chat/s/example",
            "https://www.deepseek.com/",
            "https://accounts.google.com/o/oauth2/auth",
            "https://appleid.apple.com/auth/authorize",
            "https://open.weixin.qq.com/connect/qrconnect",
            "about:blank",
            "blob:https://chat.deepseek.com/8a04055c-1f90-4cdd-9320-1a0b35dc1f00",
        ] {
            assert!(may_embed(&Url::parse(value).unwrap()), "{value}");
        }
    }

    #[test]
    fn lookalike_hosts_and_unsafe_origins_cannot_enter_the_webview() {
        for value in [
            "https://deepseek.com.evil.example/",
            "https://notdeepseek.com/",
            "https://accounts.google.com.evil.example/",
            "https://appleid.apple.com.evil.example/",
            "https://deepseek.com@evil.example/",
            "https://evil.example@chat.deepseek.com/",
            "https://chat.deepseek.com:8443/",
            "http://chat.deepseek.com/",
            "file:///etc/passwd",
            "javascript:alert(1)",
            "data:text/html,test",
            "blob:https://evil.example/example",
            "about:config",
        ] {
            assert!(!may_embed(&Url::parse(value).unwrap()), "{value}");
        }
    }

    #[test]
    fn external_references_open_in_a_browser_without_launching_arbitrary_handlers() {
        for value in ["https://example.com/", "http://example.com/"] {
            let url = Url::parse(value).unwrap();
            assert!(!may_embed(&url));
            assert!(may_open_in_browser(&url));
        }
        assert!(may_open_in_browser(
            &Url::parse("mailto:service@deepseek.com").unwrap()
        ));
        for value in [
            "file:///tmp/test",
            "javascript:alert(1)",
            "data:text/html,test",
            "shell:AppsFolder",
            "https://user:secret@example.com/",
        ] {
            assert!(!may_open_in_browser(&Url::parse(value).unwrap()), "{value}");
        }
    }
}
