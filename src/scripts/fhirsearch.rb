# File:    fhirsearch.rb
# Description: This script implements the part of a TI-Messenger-Client
#              to search for FHIR ressources in the TI FHIR-Directory.
# Parameters:  <homeserver> <mxid> <password>
# Author:'https%3A%2F%2Ffhir-directory-test.vzd.ti-dienste.de%2Fsearch%2FHealthcareService%3Forganization.active%3Dtrue%20%5C%0A%26endpoint.status%3Dactive%26_include%3DHealthcareService%3Aendpoint%26_include%3DHealthcareService%3Alocation%26%20%5C%0A_include%3DHealthcareService%3Aorganization%26_pretty%3Dtrue'
# Christian Plagens, gematik
# Version: 0.0.1

require 'faraday'
# require 'yaml'
require 'json'
require 'jwt'
require 'addressable/uri'
# require 'erb'
# include ERB::Util

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

# Messenger-Service login
url = "https://#{hs}/_matrix/client/r0/login"

conn = Faraday.new(url) do |f|
  f.request :json
  f.response :json
end

req_header = ''
req_body = { type: 'm.login.password', user: mxid, password: pw }.to_json
req_body_without_login = { type: 'm.login.password', user: 'mxid', password: 'password' }.to_json

response = conn.post do |req|
  req.headers['Content-Type'] = 'application/json'
  req_header = req.headers
  req.body = req_body
end

# log
file_content = "REQUEST\nPOST #{url}\nHEADER\n#{req_header}\nBODY\n#{req_body_without_login}\n"
file_content += "RESPONSE\nSTATUS #{response.status}\nHEADER\n#{response.headers}\n"
file_content += "BODY\n#{JSON.pretty_generate(response.body)}\n"

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

# log
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
end

# log
file_content += "REQUEST\nGET #{url}\nHEADER\n#{req_header}\n"
file_content += "RESPONSE\nSTATUS #{response.status}\nHEADER\n#{response.headers}\n"
file_content += "BODY\n#{JSON.pretty_generate(response.body)}\n"

tim_authenticate_jwt = response.body['jwt']

# FHIR /search
url = Addressable::URI.escape 'https://fhir-directory-test.vzd.ti-dienste.de/search/HealthcareService?organization.active=true&endpoint.status=active&endpoint.connection-type=tim&location.address-city=Berlin&_include=*&_count=2&_pretty=true'

conn = Faraday.new(url) do |f|
  f.request :json
  f.response :json
end

response = conn.get do |req|
  req.headers['Content-Type'] = 'application/json'
  req.headers['Authorization'] = "Bearer #{tim_authenticate_jwt}"
  req_header = req.headers
end

file_content += "REQUEST\nGET #{url}\nHEADER\n#{req_header}\n"
file_content += "RESPONSE\nSTATUS #{response.status}\nHEADER\n#{response.headers}\n"
file_content += "BODY\n#{JSON.pretty_generate(response.body)}\n"

puts file_content

# write log file
 f = File.open('/home/dev/fhirsearch_sample.txt', 'w')
 f.write(file_content)
 f.close
