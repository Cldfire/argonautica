import base64
from typing import Union

from argonautica.config import Backend, Variant, Version
from argonautica.data import RandomSalt
from argonautica.defaults import *
from argonautica._rust import ffi, lib


class Hasher:
    """
    A class that knows how to hash (but not how to verify)

    To instantiate it, just invoke it's constructor with a secret key (which can be ``None``), e.g.
    ``Hasher(secret_key=None)``. This will create a ``Hasher`` instance with the same default
    values as the ``Argon2`` class above (see above).

    You can change any one of these default values by calling the constructor with
    keyword arguments matching its properties (for a list of these properties, again,
    see ``Argon2`` above), e.g.

    .. code-block:: python

        from argonautica import Hasher

        hasher = Hasher(iterations=256, secret_key="somesecret")

    or by first instantiating a default ``Hasher`` and then modifying it's properties, e.g.

    .. code-block:: python

        from argonautica import Hasher

        hasher = Hasher(secret_key=None)
        hasher.iterations = 256
        hasher.secret_key = "somesecret"

    Once you have configured a particular ``Hasher`` instance to your liking, you can use
    it to hash by calling the ``hash`` method, e.g.

    .. code-block:: python

        from argonautica import Hasher

        hasher = Hasher(iterations=256, secret_key="somesecret")
        encoded = hasher.hash("P@ssw0rd")
        print(encoded)
    """
    __slots__ = [
        'additional_data', 'salt', 'secret_key',
        'backend', 'hash_len', 'iterations', 'lanes',
        'memory_size', 'threads', 'variant', 'version',
    ]

    def __init__(
        self,
        secret_key: Union[bytes, str, None],
        *,
        additional_data: Union[bytes, str, None] = None,
        salt: Union[bytes, RandomSalt, str] = DEFAULT_SALT,
        backend: Backend = DEFAULT_BACKEND,
        hash_len: int = DEFAULT_HASH_LEN,
        iterations: int = DEFAULT_ITERATIONS,
        lanes: int = DEFAULT_LANES,
        memory_size: int = DEFAULT_MEMORY_SIZE,
        threads: int = DEFAULT_THREADS,
        variant: Variant = DEFAULT_VARIANT,
        version: Version = DEFAULT_VERSION
    ) -> None:
        self.additional_data = additional_data
        self.salt = salt
        self.secret_key = secret_key
        self.backend = backend
        self.hash_len = hash_len
        self.iterations = iterations
        self.lanes = lanes
        self.memory_size = memory_size
        self.threads = threads
        self.variant = variant
        self.version = version

    def hash(self, password: Union[bytes, str]) -> str:
        """
        The ``hash`` method.

        This function accepts a password of type ``bytes`` or ``str`` and returns an
        encoded hash of type ``str``. The hash will be created based on the configuration of the
        ``Hasher`` instance (i.e. based on its ``salt``, ``secret_key``, ``iterations``,
        ``memory_size`` etc.).
        """
        return hash(
            password,
            additional_data=self.additional_data,
            salt=self.salt,
            secret_key=self.secret_key,
            backend=self.backend,
            hash_len=self.hash_len,
            iterations=self.iterations,
            lanes=self.lanes,
            memory_size=self.memory_size,
            threads=self.threads,
            variant=self.variant,
            version=self.version,
        )


def hash(
    password: Union[bytes, str],
    *,
    secret_key: Union[bytes, str, None],
    additional_data: Union[bytes, str, None] = None,
    salt: Union[bytes, RandomSalt, str] = DEFAULT_SALT,
    backend: Backend = DEFAULT_BACKEND,
    hash_len: int = DEFAULT_HASH_LEN,
    iterations: int = DEFAULT_ITERATIONS,
    lanes: int = DEFAULT_LANES,
    memory_size: int = DEFAULT_MEMORY_SIZE,
    threads: int = DEFAULT_THREADS,
    variant: Variant = DEFAULT_VARIANT,
    version: Version = DEFAULT_VERSION
) -> str:
    """
    A standalone hash function
    """
    data = _Data(
        additional_data=additional_data,
        password=password,
        salt=salt,
        secret_key=secret_key,
    )
    encoded_len = lib.argonautica_encoded_len(
        hash_len,
        iterations,
        lanes,
        memory_size,
        data.salt_len,
        variant.value,
    )
    encoded = ffi.new("char[]", b'\0'*encoded_len)
    err = lib.argonautica_hash(
        encoded,
        data.additional_data,
        data.additional_data_len,
        data.password,
        data.password_len,
        data.salt,
        data.salt_len,
        data.secret_key,
        data.secret_key_len,
        backend.value,
        hash_len,
        iterations,
        lanes,
        memory_size,
        0,
        0,
        threads,
        variant.value,
        version.value,
    )
    if err != lib.ARGONAUTICA_OK:
        raise Exception("failed with error code {}".format(err))
    hash = ffi.string(encoded).decode("utf-8")
    return hash


class _Data:
    def __init__(
        self,
        *,
        additional_data: Union[bytes, str, None],
        password: Union[bytes, str],
        salt: Union[bytes, RandomSalt, str],
        secret_key: Union[bytes, str, None]
    ) -> None:
        if additional_data is None:
            self.additional_data = ffi.NULL
            self.additional_data_len = 0
        elif isinstance(additional_data, bytes):
            self.additional_data = additional_data
            self.additional_data_len = len(self.additional_data)
        elif isinstance(additional_data, str):
            self.additional_data = additional_data.encode('utf-8')
            self.additional_data_len = len(self.additional_data)
        else:
            raise Exception("Error")

        if isinstance(password, bytes):
            self.password = password
            self.password_len = len(self.password)
        elif isinstance(password, str):
            self.password = password.encode("utf-8")
            self.password_len = len(self.password)
        else:
            raise Exception("TODO")

        if isinstance(salt, RandomSalt):
            self.salt = ffi.NULL
            self.salt_len = salt.len
        elif isinstance(salt, bytes):
            self.salt = salt
            self.salt_len = len(self.salt)
        elif isinstance(salt, str):
            self.salt = salt.encode('utf-8')
            self.salt_len = len(self.salt)
        else:
            raise Exception("Error")

        if secret_key is None:
            self.secret_key = ffi.NULL
            self.secret_key_len = 0
        elif isinstance(secret_key, bytes):
            self.secret_key = secret_key
            self.secret_key_len = len(self.secret_key)
        elif isinstance(secret_key, str):
            self.secret_key = secret_key.encode('utf-8')
            self.secret_key_len = len(self.secret_key)
        else:
            raise Exception("Error")
