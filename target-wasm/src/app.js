import * as golden_layout from 'https://cdn.jsdelivr.net/npm/golden-layout@2.6.0/+esm';
import init, * as incl from './target_wasm.js';

const KEY_OPERATOR_NOTATIONS = 'operator_notations';
const KEY_LOGIC = 'logic';
const KEY_PREMISES = 'premises';
const KEY_CONCLUSION = 'conclusion';
const KEY_ACTIVE_TAB_INDEX = 'activeTabIndex';

const CONFIG_KEY_MIN_COUNTERMODEL_GRAPH_NODES = 'min_countermodel_graph_nodes';
const CONFIG_KEY_MAX_COUNTERMODEL_GRAPH_NODES = 'max_countermodel_graph_nodes';
const CONFIG_KEY_SHOULD_SHUFFLE_COUNTERMODEL_GRAPHS = 'should_shuffle_countermodel_graphs';

const ID_OPERATOR_NOTATIONS_SELECT = 'operator_notations_select';
const ID_LOGICS_SELECT1 = 'logics_select1';
const ID_LOGICS_SELECT2 = 'logics_select2';
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

        show_about_panel_contents();
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
    let activeTabIndex = parseInt(localStorage.getItem(KEY_ACTIVE_TAB_INDEX)) ?? 0;
    activeTabIndex = !isNaN(activeTabIndex) ? activeTabIndex : 0;

    let config = {
        content: [
        {
            type: 'row',
            content: [
            {
                type: 'column',
                width: 35,
                content: [
                {
                    type: 'stack',
                    content: [
                    {
                        type: 'component',
                        componentName: 'ProblemComponent',
                        title: 'Problem',
                        isClosable: false,
                    },
                    {
                        type: 'component',
                        componentName: 'AboutComponent',
                        title: 'About',
                        isClosable: false,
                    }]
                },
                {
                    type: 'component',
                    componentName: 'ProblemCatalogComponent',
                    title: 'Problem Catalog',
                    height: 66,
                    isClosable: false,
                }]
            },
            {
                type: 'component',
                componentName: 'ProofTreeComponent',
                title: 'Proof Tree',
                isClosable: false,
            },
            {
                type: 'stack',
                activeItemIndex: activeTabIndex,
                content: [
                {
                    type: 'component',
                    componentName: 'ModalityGraphComponent',
                    title: 'Modality Graph',
                    isClosable: false,
                },
                {
                    type: 'component',
                    componentName: 'CountermodelGraphComponent',
                    title: 'Countermodel',
                    isClosable: false,
                },
                {
                    type: 'component',
                    componentName: 'ExecutionLogComponent',
                    title: 'Execution Log',
                    isClosable: false,
                }]
            }]
        }]
    };

    let layout = new golden_layout.GoldenLayout(config);

    layout.registerComponent('ProblemComponent', problem_component =>
    layout.registerComponent('AboutComponent', about_component =>
    layout.registerComponent('ProblemCatalogComponent', problem_catalog_component =>
    layout.registerComponent('ProofTreeComponent', proof_tree_component =>
    layout.registerComponent('ModalityGraphComponent', modality_graph_component =>
    layout.registerComponent('CountermodelGraphComponent', countermodel_graph_component =>
    layout.registerComponent('ExecutionLogComponent', execution_log_component =>
    {
        modality_graph_component.on('show', () => localStorage.setItem(KEY_ACTIVE_TAB_INDEX, 0+""))
        countermodel_graph_component.on('show', () => localStorage.setItem(KEY_ACTIVE_TAB_INDEX, 1+""));
        execution_log_component.on('show', () => localStorage.setItem(KEY_ACTIVE_TAB_INDEX, 2+""));

        callback({
            problem_input_container: problem_component.getElement(),
            problem_catalog_container: problem_catalog_component.getElement(),
            proof_tree_container: proof_tree_component.getElement(),
            modality_graph_container: modality_graph_component.getElement(),
            countermodel_graph_container: countermodel_graph_component.getElement(),
            execution_log_component: execution_log_component.getElement(),
            about_container: about_component.getElement(),
        });
    })))))));

    layout.init();
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
            div.onauxclick = () => window.open(create_shareable_url(problem), "_blank");

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
    operator_notations_select.style.width = '96.5%';
    for (let operator_notation of incl.get_operator_notations())
    {
        let option = document.createElement('option');
        option.appendChild(document.createTextNode(operator_notation));
        operator_notations_select.appendChild(option);
    }

    let logics_select1 = document.createElement('select');
    logics_select1.id = ID_LOGICS_SELECT1;
    logics_select1.style.width = '96.5%';

    let logics_select2 = document.createElement('select');
    logics_select2.id = ID_LOGICS_SELECT2;
    logics_select2.style.width = '96.5%';

    new LogicSelectGroupController(logics_select1, logics_select2).create_and_append_options();

    window.containers.problem_input_container.style.overflowY = 'scroll';
    window.containers.problem_input_container.innerHTML = `<br/>
    <table style="width: 100%; padding-left: 2%; color: white; font-size: 0.9em;">
        <tr>
            <td>Operator notations:</td>
            <td>${operator_notations_select.outerHTML}</td>
        </tr>
        <tr>
            <td>Logic:</td>
            <td>${logics_select1.outerHTML}<br/>${logics_select2.outerHTML}</td>
        </tr>
        <tr>
            <td>Premises:</td>
            <td><textarea id="${ID_PREMISES_TEXTAREA}" style="width: 95%; height: 50px;"></textarea></td>
        </tr>
        <tr>
            <td>Conclusion:</td>
            <td><textarea id="${ID_CONCLUSION_TEXTAREA}" style="width: 95%; height: 25px;"></textarea></td>
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
    let logics_select1 = document.getElementById(ID_LOGICS_SELECT1);
    let logics_select2 = document.getElementById(ID_LOGICS_SELECT2);
    let premises_textarea = document.getElementById(ID_PREMISES_TEXTAREA);
    let conclusion_textarea = document.getElementById(ID_CONCLUSION_TEXTAREA);
    let prove_button = document.getElementById(ID_PROVE_BUTTON);

    operator_notations_select.onchange = event =>
    {
        localStorage.setItem(KEY_OPERATOR_NOTATIONS, event.target.value);
        window.location = window.location.href.split("?")[0];
    };

    new LogicSelectGroupController(logics_select1, logics_select2)
        .set_on_logic_chosen(logic => update_on_screen_keyboard(logic));

    let initial_problem = {
        id: 'Initial',
        expected: '',
        logic: incl.get_logics()[0],
        premises: [],
        conclusion: '',
    };

    let url_args = new URLSearchParams(window.location.search);
    if (url_args.has(KEY_LOGIC) && url_args.has(KEY_PREMISES) && url_args.has(KEY_CONCLUSION))
    {
        initial_problem.logic = decodeURIComponent(url_args.get(KEY_LOGIC));
        initial_problem.premises = decodeURIComponent(url_args.get(KEY_PREMISES)).split('\n');
        initial_problem.conclusion = decodeURIComponent(url_args.get(KEY_CONCLUSION));
    }

    update_problem_input_area(initial_problem);

    prove_button.onclick = () =>
    {
        prove_problem({
            id: 'UserInput',
            expected: '',
            logic: new LogicSelectGroupController(logics_select1, logics_select2).get_logic(),
            premises: premises_textarea.value.split(/\r?\n/).filter(line => line.trim().length>0),
            conclusion: conclusion_textarea.value,
        });
    };

    if (url_args.has(KEY_LOGIC) && url_args.has(KEY_PREMISES) && url_args.has(KEY_CONCLUSION))
    {
        prove_button.click();
    }
}

class LogicSelectGroupController
{
    constructor(category_select, logic_select)
    {
        this.category_select = category_select;
        this.logic_select = logic_select;

        this.categories = incl.get_logics_categories();
        this.logics = incl.get_logics().filter(logic =>
            this.categories.filter(category => logic.startsWith(category)).length === 0);
    }

    create_and_append_options()
    {
        for (let category of this.categories)
        {
            let option = document.createElement('option');
            option.appendChild(document.createTextNode(category));
            this.category_select.appendChild(option);
        }

        for (let logic of this.logics)
        {
            let option = document.createElement('option');
            option.appendChild(document.createTextNode(logic));
            this.logic_select.appendChild(option);
        }
    }

    get_logic()
    {
        let category = this.categories[this.category_select.selectedIndex];
        let logic = this.logics[this.logic_select.selectedIndex];
        let category_prefix = (category === incl.get_default_logic_category()) ? '' : category+'+';
        return category_prefix + logic;
    }

    set_logic(logic)
    {
        let category = this.categories.find(category => logic.startsWith(category+'+'));
        logic = category ? logic.replace(category+'+', '') : logic;
        category = category ? category : incl.get_default_logic_category();

        this.category_select.selectedIndex = this.categories.indexOf(category);
        this.logic_select.selectedIndex = this.logics.indexOf(logic);
    }

    set_on_logic_chosen(callback)
    {
        this.category_select.onchange = event => callback(this.get_logic());
        this.logic_select.onchange = event => callback(this.get_logic());
    }
}

function update_problem_input_area(problem, proof_tree)
{
    let operator_notations_select = document.getElementById(ID_OPERATOR_NOTATIONS_SELECT);
    let logics_select1 = document.getElementById(ID_LOGICS_SELECT1);
    let logics_select2 = document.getElementById(ID_LOGICS_SELECT2);
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
    else proof_status_label.innerText = 'DISPROVED!';

    operator_notations_select.options.selectedIndex = Math.max(0,
        incl.get_operator_notations().indexOf(window.operator_notations));

    new LogicSelectGroupController(logics_select1, logics_select2).set_logic(problem.logic);

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
        render_modality_graph(proof_tree.problem.logic, proof_tree.modality_graph);
        render_countermodel_graph(proof_tree.problem.logic, proof_tree.countermodel);
        show_execution_log_panel_contents(proof_tree.execution_log);

        //changing browser URL without reloading the page, in order for the URL to be shareable
        window.history.replaceState(null, null, create_shareable_url(problem));
    }
    catch (e)
    {
        console.log(e);
        alert(e.toString());
    }
}

function create_shareable_url(problem)
{
    let operator_notations_select = document.getElementById(ID_OPERATOR_NOTATIONS_SELECT);
    let operator_notations = operator_notations_select.options[operator_notations_select.selectedIndex].text;

    let url_arguments = new URLSearchParams(window.location.search);
    url_arguments.set(KEY_OPERATOR_NOTATIONS, operator_notations);
    url_arguments.set(KEY_LOGIC, encodeURIComponent(problem.logic));
    url_arguments.set(KEY_PREMISES, encodeURIComponent(problem.premises.join("\n")));
    url_arguments.set(KEY_CONCLUSION, encodeURIComponent(problem.conclusion));
    return '?' + url_arguments.toString();
}

function render_proof_tree(proof_tree)
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

    iterate_proof_tree(proof_tree.root_node, null);

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

function render_modality_graph(logic, modality_graph)
{
    if (incl.should_skip_rendering_modality_graph(logic))
    {
        cytoscape({ container: window.containers.modality_graph_container });
        return;
    }

    let nodes = modality_graph.nodes.map((node) =>
    {
        return { data: { id:node, text:node } };
    });

    let edges = modality_graph.vertices.map((vertex) =>
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
            },
            {
                selector: 'label',
                style: {
                    'text-wrap': 'wrap',
                }
            }
        ],
    });
}

function render_countermodel_graph(logic, countermodel)
{
    window.containers.countermodel_graph_container.replaceChildren();
    if (!countermodel) { return; }

    let comment_element = document.createElement('pre');
    comment_element.style.cssText = "position:fixed; margin:1em; z-index:100; color:white; white-space:pre-wrap;";
    comment_element.append(document.createTextNode(countermodel.comment ?? ''));
    window.containers.countermodel_graph_container.appendChild(comment_element);

    if (!countermodel.was_built_from_modality_graph)
    {
        let shuffle_button = document.createElement('button');
        shuffle_button.appendChild(document.createTextNode('SHUFFLE!'));
        shuffle_button.style.cssText = "position:fixed; right:0; bottom:0; margin:1em; z-index:100; font-size:0.85em;";
        window.containers.countermodel_graph_container.appendChild(shuffle_button);

        shuffle_button.onclick = () =>
        {
            let url_arguments = new URLSearchParams(window.location.search);
            let next_number_of_nodes = parseInt(url_arguments.get(CONFIG_KEY_MIN_COUNTERMODEL_GRAPH_NODES)) === 3 ? 4 : 3;
            url_arguments.set(CONFIG_KEY_MIN_COUNTERMODEL_GRAPH_NODES, next_number_of_nodes+'');
            url_arguments.set(CONFIG_KEY_MAX_COUNTERMODEL_GRAPH_NODES, next_number_of_nodes+'');
            url_arguments.set(CONFIG_KEY_SHOULD_SHUFFLE_COUNTERMODEL_GRAPHS, true+'');
            window.history.replaceState(null, null, '?' + url_arguments.toString());

            document.getElementById(ID_PROVE_BUTTON).click();
        };
    }

    let cytoscape_graph_div = document.createElement('div');
    cytoscape_graph_div.style.width = '100%';
    cytoscape_graph_div.style.height = '100%';
    window.containers.countermodel_graph_container.appendChild(cytoscape_graph_div);

    let should_show_possible_worlds = !incl.should_skip_rendering_modality_graph(logic);
    let nodes = countermodel.nodes.map((node) =>
    {
        let text = "";
        if (should_show_possible_worlds)
        {
            text += node.possible_world.toString();
            if (!node.is_normal_world)
                text += '*';
        }

        for (let [key, value] of Object.entries(node.atomics))
            text += '\n' + key + ' : ' + value;

        return { data: { id:node.possible_world, text:text } };
    });

    let edges = countermodel.vertices.map((vertex) =>
    {
        return { data: { source:vertex.from, target:vertex.to, text:vertex.tags } };
    });

    cytoscape({
        container: cytoscape_graph_div,
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
            },
            {
                selector: 'label',
                style: {
                    'text-wrap': 'wrap',
                }
            }
        ],
    });
}

function show_execution_log_panel_contents(execution_log)
{
    window.containers.execution_log_component.style.overflowX = 'scroll';
    window.containers.execution_log_component.style.overflowY = 'scroll';

    let index = 0;
    let table_contents = '';
    for (let execution_log_line of execution_log)
    {
        table_contents += index % 4 === 0 ? '<tr>' : '';
        table_contents += '<td><pre>' + execution_log_line + '</pre></td>';
        table_contents += index % 4 === 3 ? '</tr>' : '';
        index++;
    }

    let is_benchmark_mode_enabled = window.location.toString().includes('/benchmark');
    let ram_usage_column_title = 'RAM usage' + (is_benchmark_mode_enabled ? '' :
        ' (Navigate to andob.io/incl/benchmark to enable RAM measurement!)');

    window.containers.execution_log_component.innerHTML = `
    <table style="width:100%; border-collapse:separate; border-spacing:1em; color:white; font-size:0.9em;">
        <tr>
            <td><b>Tree execution</b></td>
            <td><b>Graph execution</b></td>
            <td><b>Contradictions</b></td>
            <td><b>${ram_usage_column_title}</b></td>
        </tr>
        ${table_contents}
    </table>`;
}

function show_about_panel_contents()
{
    let book_url = 'https://www.cambridge.org/core/books/an-introduction-to-nonclassical-logic/61AD69C1D1B88006588B26C37F3A788E';
    let github_repository_url = 'https://github.com/andob/INCL-automated-theorem-prover';
    let contact_email_address = 'mc@filos.ro';

    window.containers.about_container.style.overflowY = 'scroll';
    window.containers.about_container.innerHTML = `
    <span style="color: white; padding: 1em;">
        <br/><b style="font-size: 1.1em;">INCL Automated Theorem Prover</b>
        <table style="font-size: 0.8em;"><tr>
            <td><a href="${book_url}" target="_blank"><img style="width: 150px; padding-right: 1em;" src="book.jpg" alt="Book picture"></a></td>
            <td>An automated theorem prover on first order modal logic and other non-classical formal logic systems.
                Implements the algorithm theorized by renowned philosopher and logician Graham Priest in his 2008 book
                <a href="${book_url}" target="_blank" style="color:white">"An Introduction to Non-Classical Logic. From If to Is (second edition)"</a>.
                <br/><br/>This is free open source software, licensed under the Apache License.
                This software was developed by Andrei Dobrescu and coordinated by Marian Calborean from Faculty of Philosophy, University of Bucharest.
            </td>
        </tr></table>
        <br/>
        <b>
            Contact Marian at: <a href="mailto:${contact_email_address}" target="_blank" style="color:white">mc@filos.ro</a>
            <br/>GitHub repository: <a href="${github_repository_url}" target="_blank" style="color:white">here</a>
            <br/>Version: 1.0.0
        </b>
    </span>`;
}
