# Eeyore

A lot of
projects have issue trackers that feel out of control. Consider these
other quotes, and how you feel when doing issue triage:

> Eeyore: It’s all for naught.

> Winnie the Pooh: How long will that take?  
> Eeyore: Days. Weeks. Months. Who knows?

> Eeyore: Wish I could say yes, but I can’t.  

> Eeyore: I'm telling you. People come and go in this forest, and they say,
> 'It's only Eeyore, so it doesn't count.'

> "It's snowing still," said Eeyore gloomily.  
> "So it is."  
> "And freezing."  
> "Is it?"  
> "Yes," said Eeyore. "However," he said, brightening up a little, "we
> haven't had an earthquake lately."

> "Good morning, Eeyore," said Pooh.  
> "Good morning, Pooh Bear," said Eeyore gloomily. "If it is a good
> morning, which I doubt," said he.

> Roo: Don't you ever get sick of your house falling down all the time, Eeyore?  
> Eeyore: Nope. Suppose it's just what houses do.  
> Lumpy: But houses are supposed to stay standing up.  
> Eeyore: Guess mine forgot.  

So why do issue trackers make us feel this way?

1. You can’t give people permission to only do triage, they must also have
   commit bit.
2. If you ignore your tracker, it becomes a graveyard of old issues. Feels bad.
3. Labels are wonderful, but more complex setups can be painful. For example,
   ensuring that a “Bug” and “Feature” tag are mutually exclusive, or setting up
   a pipeline of issues tracking progress.

Basically, Issues are wonderfully simple, but if you want a little bit more, it
can hurt. Eeyore can help.

## Using Eeyore

Right now, you can’t. Sorry. Still working on the initial stuff. The intention will
be to build and release something that supports giving others triage rights without
themselves needing commit bit. In other words:

1. You give Eeyore commit bit on your project.
2. You tell Eeyore who can triage your project.
3. Those users use Eeyore to do issue triage, and he does it on their behalf.

## Setting up Developer environment

- Setup a new GitHub OAuth application via this [link](https://github.com/settings/applications/new).
- Fill out all the details on the form, Authorized callback URL will be `http://localhost:3000/callback`.
- Keep the results page open, you'll need the client id and client secret for the next step.
- git clone hhttps://github.com/eeyorebot/eeyore

> Block quote are my questions for  @carol10cents and @steveklabnik.
>
> Might be a good idea to namespace env vars e.g. `EEYORE_CLIENT_ID`? The alternative would be to specify the vars at run time i.e. `SECRET=adad CLIENT_ID=asdasd cargo run`?
>

```
cat << EEYORE_EOF > ~/.env.eeyore
export SECRET=something_secure
export CLIENT_ID=github_client_id
export CLIENT_SECRET=github_secret
EEYORE_EOF
vi ~/.env.eeyore # update with correct values
source ~/.env.eeyore
```
- `cargo run`


## Acknowledgements

* Thank you to flickr user jdhancock for the [photo of eeyore](https://www.flickr.com/photos/jdhancock/7767340604)!
