# RGI

ideas:
- add factory or builder pattern to create the `Model`/`ModelRequest` for different providers with less repetitive code
- add booster lib with tricks to improve prompt performance where you can just `.boost(CoT)` a request chain
  * e.g. use `deepseek-reasoner` max-tokens 1 to build CoT for all other models
- make the API really nice, use comptime validation for all requests, prompt templates and so on
- python introperability? I have no clue how but it needs to be easy to adapt. no one will rewrite everything
- ...

# Todos
- [ ] response streaming
- [ ] Tool calling
- [ ] Completion.Choice.logprobs
