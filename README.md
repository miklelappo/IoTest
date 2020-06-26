# IoTest

IoTest is a tool to simulate IO load for memory devices


## Examples

Perform test with default settings and write output to /tmp
`IoTest --output_folder /tmp/`

Start 20 threads each of which will write 1024*1000000 bytes file to /tmp
`IoTest --output_folder /tmp/ --blocksize 1024 --blockcount=1000000 --threads 20`

## Usage

```
IoTest 0.1.0
Mikhail Lappo <mikhail.lappo@esrlabs.com>
An IO Load test

USAGE:
    IoTest [OPTIONS] --output_folder <output_folder>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --blockcount <blocknum>            Set number of blocks to be written by each thread [default: 1000]
    -s, --blocksize <blocksize>            Set blocksize [default: 512]
    -o, --output_folder <output_folder>    Select output folder
    -t, --threads <threads>                Select number of threads [default: 10]

```
