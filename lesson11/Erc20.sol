pragma solidity ^0.6.0;

import "./SafeMath.sol";



contract ERC20 {

    using SafeMath for uint256;

    

    //total supply

    uint256 private _totalSupply;

    //balance of each user

    mapping(address => uint256) _balances;

    //allowance

    mapping(address => mapping(address => uint256)) private _allowance;

    string private _name;

    string private _symbol;

    uint8 private _decimals;

    

    event Transfer(address indexed from, address indexed to, uint256 value);

    event Allowance(address indexed owner, address indexed from, uint256 value);

    event Approve(address indexed owner, address indexed from, uint256 value);

    

    constructor(string memory _myname, string memory _mysymbol, uint8 _mydecimals, uint256 _myTotalSupply) public {

        _name = _myname;

        _symbol = _mysymbol;

        _decimals = _mydecimals;

        _totalSupply = _myTotalSupply;

        _balances[msg.sender] = _totalSupply;

    }

    

    /// view function

    function name() public view returns (string memory) {

        return _name;

    }

    

    function symbol() public view returns (string memory) {

        return _symbol;

    }

    

    function totalSupply() public view returns (uint256) {

        return _totalSupply;

    }

    

    function decimals() public view returns (uint8) {

        return _decimals;

    }

    

    function balanceOf(address account) public view returns (uint256) {

        return _balances[account];

    }

    

    function allowance(address owner, address spender) public view returns (uint256) {

        return _allowance[owner][spender];

    }

    

    /// 

    function transfer(address to, uint256 value) public returns (bool) {

        return _transfer(msg.sender, to, value);

    } 

    

    function _transfer(address from, address to, uint256 value) internal returns (bool) {

        require(from != address(0), "ERC20: transfer from zero address");

        require(to != address(0), "ERC20: transfer to zero address");

        

        _balances[from] = _balances[from].sub(value);

        _balances[to] = _balances[to].add(value);

        

        emit Transfer(from, to ,value);

        

        return true;

    }

    

    function approve(address from, uint256 value) public returns (bool) {

        require(_balances[msg.sender] >= value, "ERC20: not enough money");

        

        _allowance[msg.sender][from] = value;

        

        emit Approve(msg.sender, from, value);

        

        return true;

    }

    

    function transferFrom(address owner, address to, uint256 value) public returns (bool) {

        require(_allowance[owner][msg.sender] >= value, "ERC20: not allowed to transferfrom");

        

        _allowance[owner][msg.sender] = _allowance[owner][msg.sender].sub(value);



        emit Transfer(owner, to, value);

        emit Allowance(owner, msg.sender, _allowance[owner][msg.sender]);

        

        return _transfer(owner, to, value);

    }

}
