# Seeurl


A cli written in rust to browse URLs. 

## Usage

```shell
$ seeurl --help
seeurl 0.1.0
Browses a specified url

USAGE:
    seeurl [OPTIONS] [url]

OPTIONS:
    -h, --help       Prints help information
    -v, --version    Prints version information
    -H, --headers    Specify request headers
    -o, --output     Specify output file
    -t, --timeout    Specify request timeout
    -m, --method     Specify request method
    -b, --body       Specify request body
    -d, --download   Download response

ARGS:
    <url>    Specify url to browse
```

## Installation

TODO: Create a release


### Examples

```shell
$ seeurl https://example.com
$ seeurl -H "Content-Type: application/json, Accept: application/json" -b '{"key": "value"}' -m POST https://example.com
$ seeurl -o response.json https://example.com
$ seeurl -d https://example.com
```


