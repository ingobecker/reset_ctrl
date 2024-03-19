require 'json'
require 'net/http'
require 'uri'

USER_PKG_VERS_ENDPOINT = "https://api.github.com/users/%{user}/packages/container/%{package}/versions"

def build_headers(token)
  headers = {
    Accept: 'application/vnd.github+json',
    Authorization: "Bearer #{token}",
    'X-GitHub-Api-Version': '2022-11-28',
  }
end

def get_container_vers_for_user(token, user, container)
  container = URI.encode_www_form_component(container)
  h = build_headers(token)
  u = USER_PKG_VERS_ENDPOINT % {user: user, package: container}

  puts u
  res = Net::HTTP.get_response(URI(u), h)
  if res.code != '200'
    abort("Can't load versions. Expected 200, got #{res.code}")
  end
  packages = JSON.parse(res.body)
  packages.select!{|p| p['metadata']['package_type'] == 'container'}
  packages
end

token = ENV["CONTAINER_PAT"]
user = 'ingobecker'
container = 'reset_ctrl/reset-ctrl'

puts get_container_vers_for_user(token, user, container)
