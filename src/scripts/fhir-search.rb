# File:    fhir-search.rb
# Description: This script implements the part of a TI-Messenger-Client to search for FHIR ressources in the TI FHIR-Directory.
# Author:  Christian Plagens, gematik
# Version: 0.0.1

require 'faraday'

# Messenger-Service login
conn = Faraday.new(
  url: 'https://matrix.dev.service-ti.de/_matrix/client/r0/login',
  #headers: {'Content-Type' => 'application/json'},
  headers: {'User-Agent' => 'fhir-search.rb'}
)

#response = conn.get() do |req|
#  #req.params[''],
#  req.body =
#end

response = conn.get()
puts "STATUS\n#{response.status}\n"
puts "HEADER\n#{response.headers}\n"
puts "BODY\n#{response.body}\n"
