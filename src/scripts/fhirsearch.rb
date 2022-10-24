# File:    fhirsearch.rb
# Description: This script implements the part of a TI-Messenger-Client to search for FHIR ressources in the TI FHIR-Directory.
# Author:  Christian Plagens, gematik
# Version: 0.0.1

require 'faraday'

# Messenger-Service login
conn = Faraday.new('https://matrix.dev.service-ti.de/_matrix/client/r0/login') do |f|
  f.request :json
  f.response :json
end

response = conn.post() do |req|
  req.body = { :type => 'm.login.password', :user => '@ettr02:matrix.dev.service-ti.de', :password => 'PW4ettr02' }
end

#response = conn.post()

puts "REQUEST"
puts "HEADER\n#{conn.headers}\n"
puts "BODY\n#{conn.body}\n"

puts "RESPONSE"
puts "STATUS\n#{response.status}\n"
puts "HEADER\n#{response.headers}\n"
puts "BODY\n#{response.body}\n"


