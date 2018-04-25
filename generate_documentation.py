# -*- coding: utf-8 -*-

# This script generates the API section of the documentation.

import os
import inspect
import pydoc
import sys

package_depth = 3
package_header_template = "{} {}".format(package_depth * '#','{}')

module_depth = package_depth + 1
module_header_template = "{} `{}` module".format(module_depth * '#', '{}')

class_depth = module_depth + 1
class_header_template = "{} `{}.{}`".format(class_depth * '#', '{}','{}')
class_signature_template = "class {}({})"

method_depth = class_depth + 1
method_header_template = "{} `{}`".format(method_depth * '#', '{}')

def import_module_safely(module_name):
    try:
        sys.path.append(os.getcwd())
        mod = pydoc.safeimport(module_name)
        if mod is None:
            print "Module not found ..."

        return mod

    except pydoc.ErrorDuringImport as e:
        print "Error while trying to import {} ... The docs could not be generated".format(module_name)


def generate_docs():
    package_names = ["hermes_python"]
    packages = [import_module_safely(pack_name) for pack_name in package_names]
    return getmarkdown(packages[0])


def getmarkdown(package):
    output = list()
    output.append(package_header_template.format(package.__name__))  # Adds header for module

    modules = pydoc.inspect.getmembers(package, pydoc.inspect.ismodule)  # Inspect classes
    for mod in modules:
        mod_name, mod = mod
        output.append(module_header_template.format(mod_name))

        classes = pydoc.inspect.getmembers(mod, pydoc.inspect.isclass)

        for cls_name, cls in classes:
            if cls.__module__ == (package.__name__ + "." + mod_name):
                output.append(class_header_template.format(mod_name, cls_name))  # We only take built-in classes. Not the ones that were brought into scope.
                for mth_name, mth in pydoc.inspect.getmembers(cls, pydoc.inspect.ismethod):
                    output.append(method_header_template.format(mod_name+"."+cls_name+"."+mth_name))
                    output.append("*{}*".format(pydoc.inspect.formatargspec(*pydoc.inspect.getargspec(mth))))  # Adds signature
                    if pydoc.inspect.getdoc(mth):
                        output.append("```")
                        output.append(pydoc.inspect.getdoc(mth))
                        output.append("```")
                    output.append("")

    output_md = "\n".join(output)
    return output_md


if __name__ == "__main__":
    print generate_docs()