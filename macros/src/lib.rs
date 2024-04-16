use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// # Information
/// Makes the struct serializable for `T`
#[proc_macro_derive(Serialize, attributes(serialize_as))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;

    match ast.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => {
                let field_names = fields.named.iter().map(|field| &field.ident);

                quote! {
                    impl Encoder for #struct_name {
                        async fn encode<W: tokio::io::AsyncWrite + Unpin + Send>(&self, writer: &mut W) -> Result<(), crate::errors::encode::EncodeError> {
                            #(self.#field_names.encode(writer).await?;)*
                            Ok(())
                        }
                    }
                }
            }
            .into(),
            _ => panic!("Expected a named field"),
        },
        _ => panic!("Expected a struct or an enum with named fields"),
    }
}

#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;

    match ast.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => {
                let fields = fields.named.iter().map(|field| {
                    let field_name = &field.ident.clone().unwrap();
                    let field_type = &field.ty;

                    quote! {
                        #field_name: <#field_type>::decode(reader).await?,
                    }
                });

                let gen = quote! {
                    impl crate::decoder::Decoder for #struct_name {
                        type Output = Self;

                        async fn decode<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self::Output, DecodeError> {
                            Ok(Self { #(#fields)* })
                        }
                    }
                };

                gen.into()
            }
            _ => panic!("Expected a named field"),
        },
        _ => panic!("Expected a struct with named fields"),
    }
}

#[proc_macro_derive(Networked, attributes(packet_id))]
pub fn derive_networked(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let struct_name = &ast.ident;
    let attributes = &ast.attrs;

    let attribute = attributes
        .iter()
        .filter(|a| a.path().is_ident("packet_id"))
        .nth(0)
        .expect("Expected a single numeric literal (#[packet_id(0x00)]");

    let packet_id: u8 = {
        let lit: &syn::LitInt = &attribute.parse_args().unwrap();
        let n: u8 = lit.base10_parse().expect("Expected a single numeric literal");
        n
    };

    match ast.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(fields) => {
                let field_names = fields.named.iter().map(|field| &field.ident);

                let decode_fields = fields.named.iter().map(|field| {
                    let field_name = &field.ident.clone().unwrap();
                    let field_type = &field.ty;

                    quote! {
                        #field_name: <#field_type>::decode(&mut cursor).await?,
                    }
                });

                let gen = quote! {
                    impl crate::messages::SystemPacket for #struct_name {
                        async fn to_bytes(&self) -> Result<Vec<u8>, EncodeError> {
                            let mut buffer = vec![];
                            #(self.#field_names.encode(&mut buffer).await?;)*
                            let buffer = crate::utils::prepare_response(#packet_id, buffer).await?;
                            Ok(buffer)
                        }
                    }

                    impl crate::decoder::ReceiveFromStream for #struct_name {
                        async fn from_bytes(buffer: &[u8]) -> Result<Self, crate::errors::decode::DecodeError> {
                            let mut cursor = std::io::Cursor::new(buffer);

                            Ok(Self { #(#decode_fields)* })
                        }
                    }
                };

                gen.into()
            }
            _ => panic!("Expected a named field"),
        },
        _ => panic!("Expected a struct"),
    }
}
