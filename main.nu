#!/usr/bin/env nu

def main [] {
  let endpoint = "https://lichess.org/api/account"

  curl $endpoint -H $"Authorization: Bearer ($env.LICHESS_API_TOKEN)"
  | from json
  | explore
}
