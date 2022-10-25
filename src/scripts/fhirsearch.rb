# File:    fhirsearch.rb
# Description: This script implements the part of a TI-Messenger-Client
#              to search for FHIR ressources in the TI FHIR-Directory.
# Parameters:  <homeserver> <mxid> <password>
# Author:  Christian Plagens, gematik
# Version: 0.0.1

require 'faraday'
# require 'yaml'
require 'json'
require 'jwt'

# check if mxid parameter is available
if ARGV[0] and 3 == ARGV.length then
  hs = ARGV[0]
  mxid = ARGV[1]
  pw = ARGV[2]
else
  puts 'Matrix Homeserver login'
  print 'Homeserver: '
  hs = gets.chomp
  print 'MXID: '
  mxid = gets.chomp
  print 'Password: '
  pw = gets.chomp
end

req_header = ''

# Messenger-Service login
url = "https://#{hs}/_matrix/client/r0/login"

conn = Faraday.new(url) do |f|
  f.request :json
  f.response :json
end


req_body = { type: 'm.login.password', user: mxid, password: pw }.to_json
req_body_without_login = { type: 'm.login.password', user: 'mxid', password: 'password' }.to_json

response = conn.post do |req|
  req.headers['Content-Type'] = 'application/json'
  req_header = req.headers
  req.body = req_body
end

# response = conn.post()

file_content = "REQUEST\nPOST #{url}\nHEADER\n#{req_header}\nBODY\n#{req_body_without_login}\n"
file_content += "RESPONSE\nSTATUS #{response.status}\nHEADER\n#{response.headers}\n"
file_content += "BODY\n#{JSON.pretty_generate(response.body)}\n"
# puts 'REQUEST'
# puts "HEADER\n#{conn.headers}\n"
# puts "BODY\n#{req_body}\n"

# puts 'RESPONSE'
# puts "STATUS\n#{response.status}\n"
# puts "HEADER\n#{response.headers}\n"
# puts "BODY\n#{JSON.pretty_generate(response.body)}\n"

hs_access_token = response.body['access_token']

# Matrix OpenID Token
url = "https://#{hs}/_matrix/client/v3/user/#{mxid}/openid/request_token?access_token=#{hs_access_token}"

conn = Faraday.new(url) do |f|
  f.request :json
  f.response :json
end

req_body = '{}'

response = conn.post do |req|
  req.headers['Content-Type'] = 'application/json'
  req_header = req.headers
  req.body = req_body
end

file_content += "REQUEST\nPOST #{url}\nHEADER\n#{req_header}\nBODY\n#{req_body}\n"
file_content += "RESPONSE\nSTATUS #{response.status}\nHEADER\n#{response.headers}\n"
file_content += "BODY\n#{JSON.pretty_generate(response.body)}\n"

hs_matrix_openid_token = response.body['access_token']
hs_matrix_server_name = response.body['matrix_server_name']

# tim-authenticate
url = "https://fhir-directory-test.vzd.ti-dienste.de/tim-authenticate?mxId=matrix.dev.service-ti.de"

conn = Faraday.new(url) do |f|
  f.request :json
  f.response :json
end

req_body = '{}'

response = conn.get do |req|
  req.headers['Content-Type'] = 'application/json'
  req.headers['X-Matrix-OpenID-Token'] = hs_matrix_openid_token
  req.headers['X-Matrix-Server-Name'] = hs_matrix_server_name
  req_header = req.headers
  # req.body = req_body
end

file_content += "REQUEST\nGET #{url}\nHEADER\n#{req_header}\n"
file_content += "RESPONSE\nSTATUS #{response.status}\nHEADER\n#{response.headers}\n"
file_content += "BODY\n#{JSON.pretty_generate(response.body)}\n"

tim_authenticate_jwt = response.body['jwt']

# FHIR /search
url = "https://fhir-directory-test.vzd.ti-dienste.de/search/HealthcareService?organization.active=true&_count=2"

conn = Faraday.new(url) do |f|
  f.request :json
  f.response :json
end

response = conn.get do |req|
  req.headers['Content-Type'] = 'application/json'
  req.headers['Authorization'] = "Bearer #{tim_authenticate_jwt}"
  req_header = req.headers
  # req.body = req_body
end

file_content += "REQUEST\nGET #{url}\nHEADER\n#{req_header}\n"
file_content += "RESPONSE\nSTATUS #{response.status}\nHEADER\n#{response.headers}\n"
file_content += "BODY\n#{JSON.pretty_generate(response.body)}\n"


puts file_content

f = File.open('../../samples/matrix_openid_token.txt', 'w')
f.write(file_content)
f.close
