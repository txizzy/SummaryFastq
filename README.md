# The Fastq/fq file summary tools by rust


## How to install 
1. install cargo and rust
2. git clone https://github.com/txizzy/SummaryFastq.git
3. cd SummaryFastq
4. cargo build --release

## Usage
1. For fastq file, using command:
```SummaryFastq file.fq```
2. For pigz file, using command:</br>
```pigz -dc file.fq.gz | SummaryFastq -```</br>
or </br>
```zcat file.fq.gz | SummaryFastq -```

