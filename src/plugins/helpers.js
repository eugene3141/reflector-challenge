function bigIntToFloat(number) {
    if(typeof number === "bigint") {
        let stringNumber = number.toString();

        if(stringNumber.length > 7) {
            number = `${stringNumber.slice(0, stringNumber.length - 7)}.${stringNumber.slice(-7)}`;
        } else {
            number = `0.${"0".repeat(7 - stringNumber.length)}${stringNumber}`;
        }

        return number;
    }

    return '0';
};

export default {
    install: (app, options) => {
        app.config.globalProperties.$shortAddress = (address) => {
            return address.substring(0, 4) + '...' + address.substr(address.length - 4);
        }

        app.config.globalProperties.$explorerAddress = (address) => {
            return `https://stellar.expert/explorer/testnet/${address.slice(0, 1) == 'G' ? 'account' : 'contract'}/${address}`
        }

        app.config.globalProperties.$loanStatus = (loan) => {
            if(loan.status[0] == 'WaitingForLender') return 'Pending <span class="text-xs">(Lender)</span>';

            if(loan.status[0] == 'WaitingForBorrower') return 'Pending <span class="text-xs">(Borrower)</span>';


            if(loan.status[0] == 'InProgress') {
                let seconds = Math.floor(Date.now() / 1000) - Number(loan.timestamp);
                let days = Math.ceil(seconds / 86_400);

                if(days > loan.max_loan_term) return 'Default';

                return `In Progress (${days} days)`;
            }

            return 'N/A';
        }
        
        app.config.globalProperties.$formatAmount = (amount, intOnly = false) => {
            if(typeof amount === "bigint") {
                amount = bigIntToFloat(amount);
            }

            if(typeof amount != 'string') {
                amount += '';
            }
            
            var x = amount.split('.');
            var x1 = x[0];
            var x2 = x.length > 1 ? '.' + x[1] : '';
            x2 = x2.replace(/0+$/, '');
            if(
                x2 == '.0000000'
                || x2 == '.000000'
                || x2 == '.00000'
                || x2 == '.0000'
                || x2 == '.000'
                || x2 == '.00'
                || x2 == '.0'
                || x2 == '.'
            ) x2 = '';
            var rgx = /(\d+)(\d{3})/;
            while (rgx.test(x1)) {
                x1 = x1.replace(rgx, '$1' + ',' + '$2');
            }
            return `${x1}${!intOnly ? x2 : ''}`;
        }
    }
  }