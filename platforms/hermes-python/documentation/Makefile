# Minimal makefile for Sphinx documentation
#

# You can set these variables from the command line.
SPHINXOPTS    =
SPHINXBUILD   = sphinx-build
SOURCEDIR     = source
BUILDDIR      = build

# Put it first so that "make" without argument is like "make help".
help:
	@$(SPHINXBUILD) -M help "$(SOURCEDIR)" "$(BUILDDIR)" $(SPHINXOPTS) $(O)

rst:
	@$(SPHINXBUILD) -M rst "$(SOURCEDIR)" "$(BUILDDIR)" $(SPHINXOPTS) $(O)
	head -n 9 ../README.rst >> tmp.rst
	tail -n+5 build/rst/README.rst >> tmp.rst
	mv tmp.rst ../README.rst

github:
	@$(SPHINXBUILD) -M rst "$(SOURCEDIR)" "$(BUILDDIR)" "$(SOURCEDIR)/README.rst" $(SPHINXOPTS) $(O)
	head -n 9 ../README.rst >> tmp.rst
	tail -n+5 build/rst/README.rst >> tmp.rst
	mv tmp.rst ../README.rst

.PHONY: help Makefile rst

# Catch-all target: route all unknown targets to Sphinx using the new
# "make mode" option.  $(O) is meant as a shortcut for $(SPHINXOPTS).
%: Makefile
	@$(SPHINXBUILD) -M $@ "$(SOURCEDIR)" "$(BUILDDIR)" $(SPHINXOPTS) $(O)