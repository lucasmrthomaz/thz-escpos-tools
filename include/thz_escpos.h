/**
 * thz_escpos.h
 * Header C-ABI para a biblioteca dinâmica thz-escpos-tools (thz_escpos.dll / libthz_escpos.so).
 * Motor de protocolo ESC/POS e comunicação direta com impressoras térmicas (58mm e 80mm).
 */

#ifndef THZ_ESCPOS_H
#define THZ_ESCPOS_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// Tipo opaco para o construtor ESC/POS
typedef struct EscPosBuilder EscPosBuilder;

// ============================================================================
// Ciclo de Vida do EscPosBuilder
// ============================================================================

/**
 * Cria uma nova instância do construtor de recibos ESC/POS.
 * @param columns Quantidade de colunas (32 para 58mm, 48 para 80mm).
 * @return Ponteiro para a estrutura em heap (deve ser liberada com escpos_builder_free).
 */
EscPosBuilder* escpos_builder_new(uint32_t columns);

/**
 * Libera a memória do construtor.
 */
void escpos_builder_free(EscPosBuilder* builder);

/**
 * Insere comando de inicialização da impressora (ESC @).
 */
void escpos_builder_init(EscPosBuilder* builder);

/**
 * Define a tabela de caracteres ativa (0=Cp437, 2=Cp850, 3=Cp860 Português).
 */
void escpos_builder_code_page(EscPosBuilder* builder, uint8_t page);

/**
 * Define o alinhamento horizontal (0=Esquerda, 1=Centro, 2=Direita).
 */
void escpos_builder_align(EscPosBuilder* builder, uint8_t align);

/**
 * Ativa ou desativa modo negrito (ESC E n).
 */
void escpos_builder_bold(EscPosBuilder* builder, bool enable);

/**
 * Define estilo de sublinhado (0=Nenhum, 1=Simples, 2=Duplo).
 */
void escpos_builder_underline(EscPosBuilder* builder, uint8_t style);

/**
 * Avança linhas em branco (LF).
 */
void escpos_builder_feed(EscPosBuilder* builder, uint8_t lines);

/**
 * Adiciona texto UTF-8 codificado para a Code Page ativa (sem quebra de linha).
 */
void escpos_builder_text(EscPosBuilder* builder, const char* text);

/**
 * Adiciona texto UTF-8 seguido de quebra de linha (LF).
 */
void escpos_builder_text_ln(EscPosBuilder* builder, const char* text);

/**
 * Insere linha divisória preenchida pelo caractere ch até a largura máxima.
 */
void escpos_builder_separator(EscPosBuilder* builder, char ch);

/**
 * Insere duas colunas alinhadas nos extremos da linha (ex: item à esquerda, valor à direita).
 */
void escpos_builder_two_columns(EscPosBuilder* builder, const char* left, const char* right);

/**
 * Aciona o corte de papel (0=Total, 1=Parcial, 2=Avança feed_lines linhas e corta).
 */
void escpos_builder_cut(EscPosBuilder* builder, uint8_t cut_type, uint8_t feed_lines);

/**
 * Finaliza a montagem do documento, consome o builder e retorna o buffer de bytes ESC/POS.
 * @param builder O ponteiro do builder (será liberado por esta função).
 * @param out_len Ponteiro para receber a quantidade de bytes gerados.
 * @return Ponteiro para o buffer de bytes (deve ser liberado com escpos_bytes_free).
 */
uint8_t* escpos_builder_build(EscPosBuilder* builder, size_t* out_len);

/**
 * Libera um buffer de bytes alocado por escpos_builder_build.
 */
void escpos_bytes_free(uint8_t* ptr, size_t len);

/**
 * Libera uma string alocada pelo motor.
 */
void escpos_string_free(char* ptr);

// ============================================================================
// Descoberta e Transporte Físico
// ============================================================================

/**
 * Retorna um JSON contendo o inventário de impressoras térmicas conectadas (USB / Bluetooth).
 * O ponteiro retornado DEVE ser liberado posteriormente com escpos_string_free.
 */
char* escpos_discover_printers_json(void);

/**
 * Envia bytes diretamente para a impressora USB no Windows via handle Win32 usbprint.sys.
 * @return 0 em caso de sucesso, negativo em erro.
 */
int32_t escpos_send_usb_windows(const char* device_path, const uint8_t* data, size_t len);

/**
 * Envia bytes diretamente para o nó de impressora USB no Linux (/dev/usb/lp*).
 * @return 0 em caso de sucesso, negativo em erro.
 */
int32_t escpos_send_usb_linux(const char* device_path, const uint8_t* data, size_t len);

/**
 * Envia bytes diretamente para uma porta serial RS232 ou Bluetooth SPP.
 * @return 0 em caso de sucesso, negativo em erro.
 */
int32_t escpos_send_serial(const char* port_name, uint32_t baud_rate, const uint8_t* data, size_t len);

#ifdef __cplusplus
}
#endif

#endif // THZ_ESCPOS_H
