

### Settings example


~/.my-telemetry-ui



```yaml
envs:
  env-1: ssh:gateway@10.0.0.0:22->http://10.0.0.3:8888
  env-2: ssh:gateway@10.0.0.1:22->http://10.0.0.3:8888
```


```yaml
envs:
  env-1: ssh:gateway@10.0.0.0:22->http://10.0.0.3:8888
  env-2: ssh:gateway@10.0.0.1:22->http://10.0.0.3:8888

ssh_credentials:
  "*":
    cert_path: /root/cert
    cert_pass_prase: password
```


```yaml
envs:
envs:
  env-1: ssh:gateway@10.0.0.1:22->http://10.0.0.3:8888
  env-2: ssh:gateway@10.0.0.2:22->http://10.0.0.3:8888

ssh_credentials:
  "gateway@10.0.0.0:22":
    cert_path: /root/cert-1
    cert_pass_prase: password

  "gateway@10.0.0.1:22":
    cert_path: /root/cert-2
    cert_pass_prase: password
```

ssh_credentials - can be missing. In this case SshAgent will be used.