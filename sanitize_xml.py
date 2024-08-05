from lxml import etree as ET
import re
import os

def rename_duplicates(node):
    # Initialize a dictionary to store names at each level
    names_at_level = {}
    for child in list(node):
        name_field = child.find('name')
        if name_field is not None:
            name_value = name_field.text
            if name_value in names_at_level:
                counter = names_at_level[name_value]
                new_name = f"{name_value}_{counter}"
                name_field.text = new_name
                names_at_level[name_value] += 1
            else:
                names_at_level[name_value] = 1
        rename_duplicates(child)  # Recursively check the next level

def get_name_from_description(description):
    replaces = [(',','_'),('.','_'),(':',''),(',',''),('/','_'),('#',''),('-','_'),(' ','_')]
    name = '_'.join(description.replace('-','').split()[1:4]).split('#br#')[0]
    name = name.split(':')[0]
    for r in replaces:
        name = name.replace(r[0],r[1])
    return name.upper()

if __name__=="__main__":
    out_dict = {}
    cur_dir = os.path.dirname(os.path.realpath(__file__))
    file = os.path.join(cur_dir,'xml',"iwrl6432.svd")
    # Load the XML file
    tree = ET.parse(file)
    root = tree.getroot()
    names = {}

    # Find all <field> tags
    for field in root.findall('.//field'):
        # Check if a <name> tag exists
        if field.find('name') is None:
            register = field.getparent().getparent()
            # Find the <description> tag and get its text
            description = field.find('description')
            
            if description is not None:
                name_value = get_name_from_description(description.text)
                if name_value == '-':
                    print("error")
            else:
                register = field.getparent().getparent()

                # Extract the register name
                register_name = register.find('name').text
                # Extract the bitWidth and bitOffset
                bit_width = field.find('bitWidth').text
                bit_offset = field.find('bitOffset').text
                # Format the name as desired
                name_value = f"{register_name}_W{bit_width}_O{bit_offset}"

            # Get the path to the current node
            node_path = tree.getpath(field)
            level = node_path.count('/')
            pattern = r"\[\d+\](?=[^\[]*$)"
            node_path = re.sub(pattern, "", node_path)
            if 'TPTC' in name_value:
                print(node_path)
            if node_path in names:
                if name_value in names[node_path]:
                    print(f"Duplicate name: {name_value}")
                    names[node_path][name_value] += 1
                    name_value = f"{name_value}_{names[node_path][name_value]}"
                else:
                    names[node_path][name_value] = 0
            else:
                names[node_path] = {name_value: 0}

            # Create a new <name> tag with the description text
            name = ET.SubElement(field, 'name')
            name.text = name_value
        elif ',' in field.find('name').text:
            name_value = field.find('name').text.replace(',', '')
            field.find('name').text = name_value
        else:
            name_value = field.find('name').text
    rename_duplicates(root)

    with open('xml/iwrl6432_updated.svd', 'wb') as f:
        f.write(ET.tostring(root, pretty_print=True, xml_declaration=True, encoding='utf-8'))
