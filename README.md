## Monitor de Tarefas Web em Tempo Real

Este projeto implementa um serviço de monitoramento de tarefas em tempo real, utilizando Rust para a coleta de dados do sistema e gRPC para a comunicação com um backend (a ser implementado).

**Funcionalidades:**

*   Coleta de dados do sistema em tempo real (CPU, memória, disco, processos)
*   Streaming de dados via gRPC para o backend
*   Dashboard web interativo (em desenvolvimento)
*   Fácil configuração e implantação com Docker
## Tecnologias Utilizadas

* **Backend:**
    * Node.js
    * TypeScript
    * nice-grpc (framework gRPC)
    * Firestore (banco de dados NoSQL)
* **Frontend:**
    * React
    * TypeScript
    * (Em desenvolvimento)
* **Coleta de Dados:**
    * Rust
    * sysinfo (biblioteca para informações do sistema)
    * tonic (biblioteca gRPC para Rust)
* **Comunicação:**
    * gRPC (comunicação entre o serviço Rust e o backend Node.js)
* **Infraestrutura:**
    * Docker
    * (Em desenvolvimento)


## Arquitetura

<p align="center">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://github.com/user-attachments/assets/3b8b4e42-2db3-40ad-a466-17608844c0f8">
      <source media="(prefers-color-scheme: light)" srcset="https://github.com/user-attachments/assets/3b8b4e42-2db3-40ad-a466-17608844c0f8">
      <img alt="Web-Task-Manager" title="Monitor de Tarefas Web" src="https://github.com/user-attachments/assets/3b8b4e42-2db3-40ad-a466-17608844c0f8" height="450px">
    </picture>
</p>


```
+-------------------+    +-------------------+    +-----------------+
|  Frontend (React) |    |  Backend (Node.js) |    | Rust (Coleta)   |
+-------------------+    +-------------------+    +-----------------+
      |                       |                       |
      |                       |                       |
      v                       v                       v
  +-----------+          +-----------+            +-----------+
  |   Browser |          |   gRPC    |            |   gRPC    |
  +-----------+          +-----------+            +-----------+
```

1.  **Rust (Coleta):** Coleta dados do sistema em tempo real e os envia para o backend Node.js via gRPC.
2.  **Backend (Node.js):** Recebe os dados do serviço Rust, processa e armazena no Firestore.
3.  **Frontend (React):** Exibe os dados em um dashboard interativo, consumindo a API gRPC do backend.

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
*   **0.12.0 (14/07/2024):**
    *   Versão inicial do serviço de processamento de dados em Node.js.
    *   Implementação da API gRPC para receber os dados.
    *   Streaming de dados via gRPC.
    *   Armazenamento dos dados com Firestore.
  
**Próximos Passos:**

*   Implementar a coleta de métricas de disco e rede.
*   ~~Desenvolver o backend em Node.js~~.
*   Criar o frontend em React.
*   Adicionar mais funcionalidades ao dashboard.
