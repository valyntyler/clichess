#!/usr/bin/env nu

def main [] {
  let base = "https://lichess.org/api/"
  let path = "account"

  let endpoint = $base ++ $path
  let header = $"Authorization: Bearer ($env.LICHESS_API_TOKEN)"

  curl $endpoint -H $header
  | from json
  | explore
}
