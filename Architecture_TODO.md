### Things that were implemented but need rework/fixing:

- Z-index is not working as expected.
  Need to come up with a better solution, so that: units shadows interact correctly with each other
- Shadows are disabled, as we need to find a good way to generate them automtically for entities. (I imagine there could be some for people/trees (small circular shadow under the legs/trunk), and a different kind for buildings).
- No visual design for physical entites, so their textures have different shadow styles - top-to-bottom and lop-left-to-bottom-right light sources.
- `spawn_*` methods need some better architectural ideas:
- - some sort of factory function / class or some reusable helpers, to avoid boilerplate of passing so many parameters
- - MAYBE: spawning should be invoked through a spawn event, to avoid mixing spawn calls with main business logic?
