#!/usr/bin/env nu

def main [] {
  let endpoint = "https://lichess.org/api/account"
  let header = $"Authorization: Bearer ($env.LICHESS_API_TOKEN)"

  curl $endpoint -H $header
  | from json
  | explore
}
