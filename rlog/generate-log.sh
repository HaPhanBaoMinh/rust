#!/usr/bin/env bash

set -euo pipefail

usage() {
    printf 'Usage: %s MIN_MESSAGE_CHARS MAX_MESSAGE_CHARS NUMBER_OF_LINES [OUTPUT_FILE]\n' "$(basename "$0")" >&2
    printf '\nGenerate a Lorem Ipsum log file using the project app.log format.\n' >&2
}

if [[ $# -lt 3 || $# -gt 4 ]]; then
    usage
    exit 2
fi

if [[ ! $1 =~ ^[0-9]+$ || ! $2 =~ ^[0-9]+$ ]]; then
    printf 'Error: message character limits must be non-negative integers.\n' >&2
    exit 2
fi

if [[ ! $3 =~ ^[0-9]+$ ]]; then
    printf 'Error: NUMBER_OF_LINES must be a non-negative integer.\n' >&2
    exit 2
fi

min_message_chars=$((10#$1))
max_message_chars=$((10#$2))
line_count=$((10#$3))

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
output_file=${4:-"${script_dir}/app.log"}

if (( min_message_chars > max_message_chars )); then
    printf 'Error: MIN_MESSAGE_CHARS must not exceed MAX_MESSAGE_CHARS.\n' >&2
    exit 2
fi

if [[ -z $output_file ]]; then
    printf 'Error: OUTPUT_FILE must not be empty.\n' >&2
    exit 2
fi

levels=(
    'INFO'
    'INFO'
    'WARN'
    'ERROR'
    'INFO'
    'INFO'
    'WARN'
    'INFO'
    'ERROR'
    'INFO'
)
lorem_words=(
    'lorem' 'ipsum' 'dolor' 'sit' 'amet' 'consectetur' 'adipiscing' 'elit'
    'sed' 'do' 'eiusmod' 'tempor' 'incididunt' 'ut' 'labore' 'et' 'dolore'
    'magna' 'aliqua' 'enim' 'ad' 'minim' 'veniam' 'quis' 'nostrud'
    'exercitation' 'ullamco' 'laboris' 'nisi' 'aliquip' 'ex' 'ea' 'commodo'
    'consequat' 'duis' 'aute' 'irure' 'dolor' 'in' 'reprehenderit' 'voluptate'
    'velit' 'esse' 'cillum' 'fugiat' 'nulla' 'pariatur' 'excepteur' 'sint'
    'occaecat' 'cupidatat' 'non' 'proident' 'sunt' 'in' 'culpa' 'qui'
    'officia' 'deserunt' 'mollit' 'anim' 'id' 'est' 'laborum'
)

lorem_message() {
    local target_length=$1
    local word_index=$(( $2 % ${#lorem_words[@]} ))
    local message=''
    local word

    while (( ${#message} < target_length )); do
        word=${lorem_words[$word_index]}
        if [[ -n $message ]]; then
            message+=' '
        fi
        message+=$word
        word_index=$(( (word_index + 1) % ${#lorem_words[@]} ))
    done

    printf '%s' "${message:0:target_length}"
}

temporary_file=$(mktemp "${output_file}.tmp.XXXXXX")
cleanup() {
    rm -f -- "$temporary_file"
}
trap cleanup EXIT

for ((line_number = 1; line_number <= line_count; line_number++)); do
    message_index=$(( (line_number - 1) % ${#levels[@]} ))
    if (( min_message_chars == max_message_chars )); then
        message_length=$min_message_chars
    else
        message_length=$((min_message_chars + RANDOM % (max_message_chars - min_message_chars + 1)))
    fi

    message=$(lorem_message "$message_length" "$((line_number - 1))")

    printf '%s %s event_id=%04d\n' "${levels[$message_index]}" "$message" "$line_number" >> "$temporary_file"
done

mv -- "$temporary_file" "$output_file"
trap - EXIT
printf 'Generated %d log line(s) in %s\n' "$line_count" "$output_file"
