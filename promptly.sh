#!/bin/bash

set -e

usage() {
    echo "Usage: $0 [-i|--input <string>]"
    exit 1
}

input_string=""
prompt_string=""
temperature=1.0
max_tokens=1024
top_p=1.0
while [[ $# -gt 0 ]]; do
    case "$1" in
        -i|--input)
            input_string="$2"
            shift 2
            ;;
        -p|--prompt)
            prompt_string="$2"
            shift 2
            ;;
        -t|--temperature)
            temperature="$2"
            shift 2
            ;;
        -m|--max-tokens)
            max_tokens="$2"
            shift 2
            ;;
        --top_p)
            top_p="$2"
            shift 2
            ;;
        -h|--help)
            usage
            ;;
        *)
            usage
            ;;
    esac
done

if [ -z "$input_string" ]; then
    if [ ! -t 0 ]; then
        input_string=$(cat -)
    else
        usage
    fi
fi

echo "$input_string"

json_payload=$(jq -n \
    --arg model "gpt-4o" \
    --arg role_user "user" \
    --arg user_content "$prompt_string $input_string" \
    '{
    model: $model,
    messages: [
        {
        role: $role_user,
        content: $user_content
        }
    ],
    temperature: '$temperature',
    max_tokens: '$max_tokens',
    top_p: '$top_p',
    frequency_penalty: 0,
    presence_penalty: 0
    }')

response=$(curl -s -X POST https://api.openai.com/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $OPENAI_API_KEY" \
  -d "$json_payload")
response_text=$(echo "$response" | jq -r '.choices[0].message.content')
echo $response_text