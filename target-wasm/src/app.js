import * as golden_layout from 'https://cdn.jsdelivr.net/npm/golden-layout@2.6.0/+esm';
import init, * as incl from './target_wasm.js';

const KEY_PROBLEM = 'problem';
const KEY_OPERATOR_NOTATIONS = 'operator_notations';

const ID_OPERATOR_NOTATIONS_SELECT = 'operator_notations_select';
const ID_LOGICS_SELECT = 'logics_select';
const ID_PREMISES_TEXTAREA = 'premises_textarea';
const ID_CONCLUSION_TEXTAREA = 'conclusion_textarea';
const ID_ON_SCREEN_KEYBOARD_CONTAINER = 'on_screen_keyboard_container';
const ID_PROVE_BUTTON = 'prove_button';
const ID_PROOF_STATUS_LABEL = 'proof_status_label';

(async () =>
{
    await init();
    incl.setup_console_error_panic_hook();

    initialize_operator_notations();

    initialize_layout(containers =>
    {
        window.containers = containers;

        initialize_problem_input_container();

        show_problem_catalog();
    })
})()

function initialize_operator_notations()
{
    let url_args = new URLSearchParams(window.location.search);
    let notations_from_url = url_args.get(KEY_OPERATOR_NOTATIONS);
    let notations_from_storage = localStorage.getItem(KEY_OPERATOR_NOTATIONS);
    window.operator_notations = notations_from_url ?? (notations_from_storage ?? '');
    incl.set_operator_notations(window.operator_notations);
}

function initialize_layout(callback)
{
    let config = {
        content: [
            {
                type: 'row',
                content: [
                    {
                        type: 'column',
                        content: [
                            {
                                type: 'component',
                                componentName: 'ProblemComponent',
                                title: 'Problem',
                                isClosable: false,
                            },
                            {
                                type: 'component',
                                componentName: 'ProblemCatalogComponent',
                                title: 'Problem Catalog',
                                isClosable: false,
                            }
                        ]
                    },
                    {
                        type: 'component',
                        componentName: 'ProofTreeComponent',
                        title: 'Proof Tree',
                        isClosable: false,
                    },
                    {
                        type: 'stack',
                        content: [
                            {
                                type: 'component',
                                componentName: 'ModalityGraphComponent',
                                title: 'Modality Graph',
                                isClosable: false,
                            }
                        ]
                    }
                ]
            }]
    };

    let layout = new golden_layout.GoldenLayout(config);

    layout.registerComponent('ProblemComponent', problem_component =>
    layout.registerComponent('ProblemCatalogComponent', problem_catalog_component =>
    layout.registerComponent('ProofTreeComponent', proof_tree_component =>
    layout.registerComponent('ModalityGraphComponent', modality_graph_component =>
    callback({
        problem_input_container: problem_component.getElement(),
        problem_catalog_container: problem_catalog_component.getElement(),
        proof_tree_container: proof_tree_component.getElement(),
        modality_graph_container: modality_graph_component.getElement(),
    })))));

    layout.init();
}

function render_proof_tree(json)
{
    let nodes = [], edges = [];

    function iterate_proof_tree(node, parent_id)
    {
        let node_data = {
            id: node.id,
            text: node.formula + (node.is_contradictory ? " X" : ""),
            spawner_node_id: node.spawner_node_id,
            contrarian_node_id: node.contrarian_node_id
        };

        nodes.push({ data: node_data });

        if (parent_id != null)
        {
            let vertex_data = { source: parent_id, target: node.id };
            edges.push({ data: vertex_data });
        }

        if (node.left != null) { iterate_proof_tree(node.left, node.id); }
        if (node.middle != null) { iterate_proof_tree(node.middle, node.id); }
        if (node.right != null) { iterate_proof_tree(node.right, node.id); }
    }

    iterate_proof_tree(json.root_node, null);

    let cy = cytoscape({
        container: window.containers.proof_tree_container,
        elements: {
            nodes: nodes,
            edges: edges,
        },
        layout: {
            name: 'dagre',
        },
        wheelSensitivity: 0.1,
        style: [
            {
                selector: 'node',
                style: {
                    'background-color': 'black',
                    'text-valign': 'center',
                    'text-halign': 'center',
                    'color': 'white',
                    'label': 'data(text)',
                }
            },
            {
                selector: 'label',
                style: {
                    'text-wrap': 'wrap',
                }
            }
        ],
    });

    function unselect_all_nodes()
    {
        cy.nodes().forEach((node) => node.style("background-color", "black"));
    }

    function select_node(tapped_node)
    {
        tapped_node.style("background-color", "#119d11");

        let contrarian_node_id = tapped_node.data().contrarian_node_id;
        if (contrarian_node_id != null)
        {
            let contrarian_node = cy.nodes().find((node) => parseInt(node.data().id) === parseInt(contrarian_node_id));

            contrarian_node.style("background-color", "#595959");
        }

        let spawner_node_id = tapped_node.data().spawner_node_id;
        if (spawner_node_id != null)
        {
            let spawner_node = cy.nodes().find((node) => parseInt(node.data().id) === parseInt(spawner_node_id));
            let spawned_nodes = cy.nodes().filter((node) => parseInt(node.data().spawner_node_id) === parseInt(spawner_node_id));

            spawner_node.style("background-color", "#b61212");

            spawned_nodes.filter((node) => parseInt(node.data().id) !== parseInt(tapped_node.data().id))
                .forEach((node) => node.style("background-color", "#1212b6"));
        }
    }

    let is_node_hovering_enabled = true;
    cy.on('tap', 'node', (event) =>
    {
        is_node_hovering_enabled = false;
        unselect_all_nodes();
        select_node(event.target);
    });

    cy.on('tap', (event) =>
    {
        if (event.target === cy)
        {
            //tapped outside the node
            is_node_hovering_enabled = true;
            unselect_all_nodes();
        }
    });

    cy.on('mouseover', 'node', (event) =>
    {
        if (is_node_hovering_enabled)
        {
            unselect_all_nodes();
            select_node(event.target);
        }
    });

    cy.on('mouseout', 'node', (event) =>
    {
        if (is_node_hovering_enabled)
        {
            unselect_all_nodes();
        }
    });
}

function render_modality_graph(json)
{
    if (incl.should_skip_rendering_modality_graph(json.problem.logic))
    {
        cytoscape({ container: window.containers.modality_graph_container });
        return;
    }

    let nodes = json.modality_graph.nodes.map((node) =>
    {
        return { data: { id:node, text:node } };
    });

    let edges = json.modality_graph.vertices.map((vertex) =>
    {
        return { data: { source:vertex.from, target:vertex.to, text:vertex.tags } };
    });

    cytoscape({
        container: window.containers.modality_graph_container,
        elements: {
            nodes: nodes,
            edges: edges,
        },
        layout: {
            name: 'avsdf',
            animationDuration: 0,
            nodeSeparation: 120
        },
        wheelSensitivity: 0.1,
        style: [
            {
                selector: 'node',
                style: {
                    'background-color': 'black',
                    'text-valign': 'center',
                    'text-halign': 'center',
                    'color': 'white',
                    'label': 'data(text)',
                }
            },
            {
                selector: 'edge',
                style: {
                    'target-arrow-shape': 'triangle',
                    'curve-style': 'bezier',
                    'text-valign': 'center',
                    'text-halign': 'center',
                    'color': 'white',
                    'label': 'data(text)',
                    'width': 1.0,
                }
            }
        ],
    });
}

function show_problem_catalog()
{
    window.containers.problem_catalog_container.style.overflowY = 'scroll';
    window.containers.problem_catalog_container.innerHTML = '';

    let book_chapters_ol = document.createElement("ol");
    book_chapters_ol.style.color = 'white';
    book_chapters_ol.style.fontSize = '0.8em';

    let problem_catalog = JSON.parse(incl.get_problem_catalog());
    for (let book_chapter of problem_catalog)
    {
        let book_chapter_li = document.createElement("li");
        book_chapter_li.appendChild(document.createTextNode(book_chapter.name));
        book_chapter_li.appendChild(document.createElement("br"));

        for (let problem of book_chapter.problems)
        {
            let div = document.createElement("div");
            div.style.textDecoration = 'underline';
            div.style.display = 'inline';
            div.style.cursor = 'pointer';

            div.appendChild(document.createTextNode(problem.id));
            div.onclick = () => prove_problem(problem);

            book_chapter_li.appendChild(div);
            book_chapter_li.appendChild(document.createTextNode("  "))
        }

        book_chapter_li.appendChild(document.createElement("br"));
        book_chapters_ol.appendChild(book_chapter_li);
    }

    window.containers.problem_catalog_container.appendChild(book_chapters_ol);
}

function initialize_problem_input_container()
{
    let operator_notations_select = document.createElement('select');
    operator_notations_select.id = ID_OPERATOR_NOTATIONS_SELECT;
    for (let operator_notation of incl.get_operator_notations())
    {
        let option = document.createElement('option');
        option.appendChild(document.createTextNode(operator_notation));
        operator_notations_select.appendChild(option);
    }

    let logics_select = document.createElement('select');
    logics_select.id = ID_LOGICS_SELECT;
    for (let logic of incl.get_logics())
    {
        let option = document.createElement('option');
        option.appendChild(document.createTextNode(logic));
        logics_select.appendChild(option);
    }

    window.containers.problem_input_container.style.overflowY = 'scroll';
    window.containers.problem_input_container.innerHTML = `<br/>
    <table style="width: 100%; color: white; font-size: 0.9em;">
        <tr>
            <td>Operator notations:</td>
            <td>${operator_notations_select.outerHTML}</td>
        </tr>
        <tr>
            <td>Logic:</td>
            <td>${logics_select.outerHTML}</td>
        </tr>
        <tr>
            <td>Premises:</td>
            <td><textarea id="${ID_PREMISES_TEXTAREA}" style="width: 450px; height: 50px;"></textarea></td>
        </tr>
        <tr>
            <td>Conclusion:</td>
            <td><textarea id="${ID_CONCLUSION_TEXTAREA}" style="width: 450px; height: 25px;"></textarea></td>
        </tr>
        <tr>
            <td></td>
            <td><div id="${ID_ON_SCREEN_KEYBOARD_CONTAINER}"></div></td>
        </tr>
        <tr>
            <td><br/><button id="${ID_PROVE_BUTTON}" style="font-size: 1.1em; font-style: italic; width: 75px; height: 50px;">PROVE!</button></td>
            <td><br/><b id="${ID_PROOF_STATUS_LABEL}" style="font-size: 1.1em; font-style: italic; width: 75px; height: 50px;"><b></td>
        </tr>
    </table>`;

    setTimeout(initialize_problem_input_container_after_delay, 10);
}

function initialize_problem_input_container_after_delay()
{
    let operator_notations_select = document.getElementById(ID_OPERATOR_NOTATIONS_SELECT);
    let logics_select = document.getElementById(ID_LOGICS_SELECT);
    let premises_textarea = document.getElementById(ID_PREMISES_TEXTAREA);
    let conclusion_textarea = document.getElementById(ID_CONCLUSION_TEXTAREA);
    let prove_button = document.getElementById(ID_PROVE_BUTTON);

    operator_notations_select.onchange = event =>
    {
        localStorage.setItem(KEY_OPERATOR_NOTATIONS, event.target.value);
        window.location = window.location.href.split("?")[0];
    };

    logics_select.onchange = event => update_on_screen_keyboard(event.target.value);

    let initial_problem = {
        id: 'Initial',
        expected: '',
        logic: incl.get_logics()[0],
        premises: [],
        conclusion: '',
    };

    let url_args = new URLSearchParams(window.location.search);
    if (url_args.has(KEY_PROBLEM))
    {
        let problem = JSON.parse(incl.get_problem_catalog())
            .flatMap(chapter => chapter.problems)
            .find(problem => problem.id === url_args.get(KEY_PROBLEM).toString())
        initial_problem = problem ? problem : initial_problem;
    }

    update_problem_input_area(initial_problem);

    prove_button.onclick = () =>
    {
        prove_problem({
            id: 'UserInput',
            expected: '',
            logic: logics_select.value,
            premises: premises_textarea.value.split(/\r?\n/).filter(line => line.trim().length>0),
            conclusion: conclusion_textarea.value,
        });
    };

    if (url_args.has(KEY_PROBLEM))
    {
        prove_button.click();
    }
}

function update_problem_input_area(problem, proof_tree)
{
    let operator_notations_select = document.getElementById(ID_OPERATOR_NOTATIONS_SELECT);
    let logics_select = document.getElementById(ID_LOGICS_SELECT);
    let premises_textarea = document.getElementById(ID_PREMISES_TEXTAREA);
    let conclusion_textarea = document.getElementById(ID_CONCLUSION_TEXTAREA);
    let proof_status_label = document.getElementById(ID_PROOF_STATUS_LABEL);

    premises_textarea.value = problem.premises.join('\n');
    conclusion_textarea.value = problem.conclusion;

    if (!proof_tree)
        proof_status_label.innerText = '';
    else if (proof_tree.has_timeout)
        proof_status_label.innerText = 'TIMEOUT!';
    else if (proof_tree.was_proved)
        proof_status_label.innerText = 'PROVED!';
    else proof_status_label.innerText = 'NOT PROVED!';

    operator_notations_select.options.selectedIndex = Math.max(0,
        incl.get_operator_notations().indexOf(window.operator_notations));

    logics_select.options.selectedIndex = incl.get_logics().indexOf(problem.logic);

    update_on_screen_keyboard(problem.logic);
}

function update_on_screen_keyboard(logic)
{
    let on_screen_keyboard_container = document.getElementById(ID_ON_SCREEN_KEYBOARD_CONTAINER);
    on_screen_keyboard_container.innerHTML = '';

    let conclusion_textarea = document.getElementById(ID_CONCLUSION_TEXTAREA);
    let premises_textarea = document.getElementById(ID_PREMISES_TEXTAREA);

    let focused_textarea = conclusion_textarea;
    premises_textarea.onfocus = () => { focused_textarea = premises_textarea; }
    conclusion_textarea.onfocus = () => { focused_textarea = conclusion_textarea; }

    for (let symbol of incl.get_operator_symbols(logic))
    {
        let symbol_button = document.createElement('button');
        symbol_button.appendChild(document.createTextNode(symbol));

        symbol_button.onclick = () =>
        {
            let position = focused_textarea.selectionStart;
            let before = focused_textarea.value.substring(0, position);
            let after = focused_textarea.value.substring(position, focused_textarea.value.length);
            focused_textarea.value = before + symbol_button.textContent + after;
            focused_textarea.selectionStart = focused_textarea.selectionEnd = position + symbol_button.textContent.length;
            focused_textarea.focus();
        };

        on_screen_keyboard_container.appendChild(symbol_button);
    }
}

function prove_problem(problem)
{
    try
    {
        let problem_json = JSON.stringify(problem);
        let proof_tree_json = incl.solve_problem(problem_json);
        let proof_tree = JSON.parse(proof_tree_json);

        update_problem_input_area(problem, proof_tree);

        render_proof_tree(proof_tree);
        render_modality_graph(proof_tree);
    }
    catch (e)
    {
        console.log(e);
        alert(e.toString());
    }
}
