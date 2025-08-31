#!/usr/bin/env nu

def main [] {
  let header = $"Authorization: Bearer ($env.LICHESS_API_TOKEN)"

  let url = {
    scheme: https
    host: lichess.org
    path: api/account
  }

  curl ($url | url join) -H $header
  | from json
  | explore
}
