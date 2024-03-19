require 'json'
require 'net/http'
require 'uri'

USER_PKG_VERS_ENDPOINT = "https://api.github.com/users/%{user}/packages/container/%{package}/versions"
USER_PKG_VERS_DEL_ENDPOINT = "https://api.github.com/users/%{user}/packages/container/%{package}/versions/%{ver}"

def build_headers(token)
  headers = {
    Accept: 'application/vnd.github+json',
    Authorization: "Bearer #{token}",
    'X-GitHub-Api-Version': '2022-11-28',
  }
end

def get_container_vers_for_user(token, user, package)
  package = URI.encode_www_form_component(package)
  h = build_headers(token)
  u = USER_PKG_VERS_ENDPOINT % {user: user, package: package}

  res = Net::HTTP.get_response(URI(u), h)
  if res.code != '200'
    abort("Can't load versions. Expected 200, got #{res.code}")
  end
  packages = JSON.parse(res.body)
  packages.select!{|p| p['metadata']['package_type'] == 'container'}
  packages
end

def del_container_vers_for_user(token, user, package, ver)
  package = URI.encode_www_form_component(package)
  h = build_headers(token)
  e = USER_PKG_VERS_DEL_ENDPOINT % {user: user, package: package, ver: ver}
  u = URI(e)

  Net::HTTP.start(u.host, u.port, use_ssl: true) do |http|
    res = http.delete(u, h)
    if res.code != '204'
      abort("Can't delete version #{ver}. Expected 204, got #{res.code}")
    end
  end
end

def get_untagged_containers(containers)
  containers.select{|c| c['metadata']['container']['tags'].empty?}
end

def get_containers_by_tag(containers, tag)
  containers.select{|c| c['metadata']['container']['tags'].include?(tag)}
end


token = ENV["CONTAINER_PAT"]
user = 'ingobecker'
container = 'reset_ctrl/reset-ctrl'
pr_container_tag = ENV["IMAGE_TAG"] + ENV["IMAGE_TAG_PR"]
delete_pr_container = ENV["DELETE_PR_CONTAINER"]

containers = get_container_vers_for_user(token, user, container)
untagged_containers = get_untagged_containers(containers)

puts "Running container GC"
puts "PR container tag: #{pr_container_tag}"
puts "Delete pr container: #{delete_pr_container}"
puts "Total number of untagged containers: #{untagged_containers.count}"
puts "Total number of containers: #{containers.count}"
puts "\n\n"

untagged_containers.each do |c|
  ver = c["id"]
  sha = c["name"]
  puts "Deleting untagged container #{sha} (id: #{ver})"
  del_container_vers_for_user(token, user, container, ver)
end

if delete_pr_container == 'true'
  puts "PR is closed, trying to delete container with tag '#{pr_container_tag}'"
  pr_container = get_containers_by_tag(containers, pr_container_tag)

  if pr_container.any?
    puts "Container with tag '#{pr_container_tag}' found!"
    pr_container.each do |c|
      ver = c["id"]
      sha = c["name"]
      puts "Deleting pr container #{sha} (id: #{ver})"
      del_container_vers_for_user(token, user, container, ver)
    end
  end
end
