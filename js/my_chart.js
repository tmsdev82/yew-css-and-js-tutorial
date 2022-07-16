export class MyChart {
    constructor() {    
        this.chart = null;    
        let data = {
            labels: [],
            datasets: [{
                label: 'Widget data',
                backgroundColor: 'rgb(255, 99, 132)',
                borderColor: 'rgb(255, 99, 132)',
                data: [],
            }]
        };

        this.config = {
            type: 'line',
            data: data,
            options: {
                responsive: false,
                scales: {
                    y: {
                        suggestedMin: 0,
                        suggestedMax: 50
                    }
                }
            }
        };
    }

    draw(element_id) {
        this.chart = new Chart(
            document.getElementById(element_id),
            this.config
        )
    }

    update(value) {
        let labels = this.chart.data.labels;
        console.log("Updating chart labels");
        labels.push(labels.length+1);
        console.log("Updating chart data");
        this.chart.data.datasets[0].data.push(value);
        this.chart.update()
    }
}
