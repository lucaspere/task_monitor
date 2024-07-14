## Monitor de Tarefas Web em Rust e gRPC

Este projeto implementa um serviço de monitoramento de tarefas em tempo real, utilizando Rust para a coleta de dados do sistema e gRPC para a comunicação com um backend (a ser implementado).

**Funcionalidades:**

*   Coleta de dados do sistema em tempo real (CPU, memória, disco, processos)
*   Streaming de dados via gRPC para o backend
*   Dashboard web interativo (em desenvolvimento)
*   Fácil configuração e implantação com Docker

**Tecnologias:**

*   **Rust:** Linguagem de programação utilizada para a implementação do serviço.
*   **gRPC:** Para comunicação eficiente entre o serviço Rust e o futuro backend.
*   **sysinfo:** Biblioteca Rust para obter informações do sistema.
*   **tokio:** Runtime assíncrono para Rust.
*   **tonic:** Implementação do gRPC para Rust.
*   **serde:** Framework para serialização e desserialização de dados.
*   **chrono:** Biblioteca para manipulação de datas e horas.
*   **Rust:** Para o serviço de coleta de dados, garantindo desempenho e segurança.
*   **Docker:** Para facilitar a implantação e a portabilidade.
*   **(Futuras)** React, Node.js, GCP: Para o desenvolvimento do backend e do frontend.

**Como Executar:**
**Instalação:**

1.  Clone o repositório.
2.  Execute `docker build -t task-monitor-service .` para construir a imagem Docker.
3.  Execute `docker run -d --name task-monitor task-monitor-service` para iniciar o serviço.

**Changelog:**

*   **0.1.0 (14/07/2024):**
    *   Versão inicial do serviço de coleta de dados em Rust.
    *   Implementação da coleta de métricas de CPU e memória.
    *   Streaming de dados via gRPC.
    *   Dockerização do serviço.
  
**Próximos Passos:**

*   Implementar a coleta de métricas de disco e rede.
*   Desenvolver o backend em Node.js.
*   Criar o frontend em React.
*   Adicionar mais funcionalidades ao dashboard.




