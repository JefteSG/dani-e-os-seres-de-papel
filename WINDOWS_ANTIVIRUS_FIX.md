# 🔒 Solução para Problemas de Antivírus no Windows

## ⚠️ Problema Comum
Alguns antivírus (incluindo Windows Defender) podem detectar falsos positivos em executáveis compilados em Rust. Isso é normal e pode ser resolvido facilmente.

## 🛡️ Soluções

### **Opção 1: Adicionar Exceção no Windows Defender**

1. **Abra o Windows Defender**:
   - Pressione `Windows + I` para abrir Configurações
   - Vá em "Atualização e Segurança" → "Windows Defender"
   - Clique em "Abrir Windows Defender"

2. **Adicione Exceção**:
   - Clique em "Configurações de proteção contra vírus e ameaças"
   - Role para baixo e clique em "Configurações de proteção contra vírus e ameaças"
   - Em "Exclusões", clique em "Adicionar ou remover exclusões"
   - Clique em "Adicionar uma exclusão" → "Pasta"
   - Selecione a pasta onde você extraiu o jogo

### **Opção 2: Verificar Arquivo no Windows Defender**

1. **Clique com botão direito** no arquivo `dani_seres_do_papel.exe`
2. Selecione **"Verificar com Windows Defender"**
3. Se aparecer "Nenhuma ameaça encontrada", o arquivo é seguro

### **Opção 3: Desabilitar Temporariamente o Antivírus**

⚠️ **ATENÇÃO**: Use apenas se confiar na fonte do arquivo

1. **Windows Defender**:
   - Configurações → Atualização e Segurança → Windows Defender
   - Desative temporariamente a proteção em tempo real
   - Execute o jogo
   - Reative a proteção

2. **Outros Antivírus**:
   - Verifique as configurações do seu antivírus
   - Adicione o arquivo ou pasta às exceções

## 🔍 Por que isso acontece?

### **Causas Comuns**:
- **Executáveis não assinados**: Compilados sem certificado digital
- **Comportamento suspeito**: Acesso a arquivos do sistema
- **Heurística**: Detecção baseada em padrões genéricos
- **Falsos positivos**: Antivírus muito agressivo

### **Por que é seguro**:
- **Código aberto**: Todo o código está disponível no GitHub
- **Compilação limpa**: Sem malware ou código malicioso
- **Comunidade**: Verificado por desenvolvedores

## 📋 Verificação de Segurança

### **Antes de Executar**:
1. ✅ **Fonte confiável**: Download do GitHub oficial
2. ✅ **Código aberto**: Pode ser auditado
3. ✅ **SHA256**: Verifique o hash do arquivo
4. ✅ **Antivírus**: Execute verificação completa

### **Hash do Arquivo**:
```
# Windows v1.0.5
SHA256: d859d33a0387001fac91e7ebfc...
```

## 🚀 Alternativas

### **Compilação Local**:
Se ainda tiver problemas, compile localmente:
```bash
# Instale o Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone e compile
git clone https://github.com/JefteSG/dani-e-os-seres-de-papel.git
cd dani-e-os-seres-de-papel
cargo build --release --target x86_64-pc-windows-msvc
```

### **Executar via Terminal**:
```cmd
# Abra o CMD como administrador
cd C:\caminho\para\o\jogo
dani_seres_do_papel.exe
```

## 📞 Suporte

Se ainda tiver problemas:
1. **GitHub Issues**: [Reporte o problema](https://github.com/JefteSG/dani-e-os-seres-de-papel/issues)
2. **Verificação**: Execute `virustotal.com` no arquivo
3. **Comunidade**: Peça ajuda na comunidade Rust

---

**Lembre-se**: Este é um jogo de código aberto e seguro. Os falsos positivos são comuns em executáveis não assinados. 