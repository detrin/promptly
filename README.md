# promptly
A CLI tool for processing data with LLMs.

Currently supports only openai.

## Example of use

Set the openai key variable
```
OPENAI_API_KEY="*********************"
```
Text summarization
```
cargo run -- --prompt "Summarize the following text: " --input "In the atrium of a research building at the Chinese Academy of Sciences (cas) in Beijing is a wall of patents. Around five metres wide and two storeys high, the wall displays 192 certificates, positioned in neat rows and tastefully lit from behind. At ground level, behind a velvet rope, an array of glass jars contain the innovations that the patents protect: seeds.
cas—the world’s largest research organisation—and institutions around China produce a huge amount of research into the biology of food crops. In the past few years Chinese scientists have discovered a gene that, when removed, boosts the length and weight of wheat grains, another that improves the ability of crops like sorghum and millet to grow in salty soils and one that can increase the yield of maize by around 10%. In autumn last year, farmers in Guizhou completed the second harvest of genetically modified giant rice that was developed by scientists at cas."
```
```
At the Chinese Academy of Sciences (CAS) in Beijing, a prominent wall displays 192 patents related to agricultural innovations. This wall is dedicated to showcasing research on food crop biology. Chinese scientists have recently made significant discoveries, including genes that enhance wheat grains, improve crop growth in salty soils, and increase maize yield by 10%. Researchers also developed genetically modified giant rice, with successful harvests reported in Guizhou. CAS, the world's largest research organization, plays a central role in these advancements.
```
Processing in parallel with prime numbers example and extracting the json from text reponse
```
seq 1 20 | parallel -j10 'response=$(./promptly -p "Is the following number a prime? Respond with JSON \`\`\`json {\"is_prime\": boolean}\`\`\`. Do not include any explanation." -i {} --max-tokens 24 --extract-json | jq ".is_prime"); echo {} $response'
```
```
3 true
5 true
2 true
7 true
4 false
8 false
9 false
6 false
1 false
10 false
11 true
14 false
12 false
15 false
13 true
16 false
19 true
20 false
18 false
17 true
```

## TODO
* JSON parsing from response
* text to speech from openai API
* added support for also other LLMs or platforms