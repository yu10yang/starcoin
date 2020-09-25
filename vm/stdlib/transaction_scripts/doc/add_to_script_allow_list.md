
<a name="SCRIPT"></a>

# Script `add_to_script_allow_list.move`

### Table of Contents

-  [Function `add_to_script_allow_list`](#SCRIPT_add_to_script_allow_list)
-  [Specification](#SCRIPT_Specification)
    -  [Function `add_to_script_allow_list`](#SCRIPT_Specification_add_to_script_allow_list)



<a name="SCRIPT_add_to_script_allow_list"></a>

## Function `add_to_script_allow_list`

Append the <code>hash</code> to script hashes list allowed to be executed by the network.
Todo: it's dangous to run the script when publish option is VMPublishingOption::Open
because the list is empty at the moment, adding script into the empty list will lead to
that only the added script is allowed to execute.


<pre><code><b>public</b> <b>fun</b> <a href="#SCRIPT_add_to_script_allow_list">add_to_script_allow_list</a>(account: &signer, hash: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="#SCRIPT_add_to_script_allow_list">add_to_script_allow_list</a>(account: &signer, hash: vector&lt;u8&gt;) {
    <a href="../../modules/doc/TransactionPublishOption.md#0x1_TransactionPublishOption_add_to_script_allow_list">TransactionPublishOption::add_to_script_allow_list</a>(account, hash)
}
</code></pre>



</details>

<a name="SCRIPT_Specification"></a>

## Specification


<a name="SCRIPT_Specification_add_to_script_allow_list"></a>

### Function `add_to_script_allow_list`


<pre><code><b>public</b> <b>fun</b> <a href="#SCRIPT_add_to_script_allow_list">add_to_script_allow_list</a>(account: &signer, hash: vector&lt;u8&gt;)
</code></pre>




<pre><code>pragma verify = <b>false</b>;
</code></pre>