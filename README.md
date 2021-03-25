The Domain-to-IP binary file helps you to get ip of a list of domains and print them on your screen.
Usage:

```bash
cat domains.txt | ./domain-to-ip
```

If you'd like to use Cloudflare's 1.1.1.1 DNS service add `secure` keyword in your command line.

```bash
cat domains.txt | ./domain-to-ip secure
```

Check for errors in output by adding `debug` keyword in your command line:

```bash
cat domains.txt | ./domain-to-ip
```
