class RandomSalt:
    def __init__(self, len: int = 32) -> None:
        self.len = len


DEFAULT_SALT = RandomSalt(32)
