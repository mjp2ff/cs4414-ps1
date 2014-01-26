ps1
===

Completed PS1 for CS4414, Spring 2014.

In addition to the assignment requirements, I've also verified that a file exists before attempting to load it. To avoid giving too much information about which files exist on the system, 403 errors take precedence over 404s. That means that if a non-HTML file is requested, whether or not it exists, a 403 error is returned. The only time a 404 is returned is if an HTML file that does not exist is requested.