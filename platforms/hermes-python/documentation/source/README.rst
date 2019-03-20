Hermes Python
=============
.. include:: about.rst

.. include:: requirements.rst

.. include:: installation.rst

.. include:: tutorial.rst

Release Checklist
=================

Everytime you need to perform a release, do the following steps :
- [ ] Commit all changes to the project for said release
- [ ] Write all the changes introduced in the Changelog (source/HISTORY.rst file) and commit it
- [ ] Run tests
- [ ] Build the documentation and commit the README.rst
- [ ] Bump the version and commit it
- [ ] Upload to PyPI

Build details
=============

Creating macOS wheels
---------------------

The build script : ``build_scripts/build_macos_wheels.sh`` uses ``pyenv`` to generate ``hermes-python``
wheels for different versions of python.

To be able to run it, you need to :

- install pyenv : brew install pyenv. Then follow the additional steps detailled
- you then have to install python at different versions:  ``pyenv install --list`` to list the available version to install
- Before installing and building the different python version from sources, install the required dependencies : `Link here <https://github.com/pyenv/pyenv/wiki />`_

That's it !
