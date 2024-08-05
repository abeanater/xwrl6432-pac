#[doc = "Register `CIDX` reader"]
pub type R = crate::R<CidxSpec>;
#[doc = "Register `CIDX` writer"]
pub type W = crate::W<CidxSpec>;
#[doc = "Field `SCIDX` reader - 15:0\\]
Source Frame Index: SCIDX is a 16-bit signed value (2's complement) used for source address modification for the 3rd dimension. It is a signed value between -32768 and 32767. It provides a byte address offset from the beginning of the current array (pointed to by SRC address) to the beginning of the first source array in the next frame. It applies to both A-sync and AB-sync transfers. Note that when SCIDX is applied the current array in an A-sync transfer is the last array in the frame while the current array in a AB-sync transfer is the first array in the frame."]
pub type ScidxR = crate::FieldReader<u16>;
#[doc = "Field `SCIDX` writer - 15:0\\]
Source Frame Index: SCIDX is a 16-bit signed value (2's complement) used for source address modification for the 3rd dimension. It is a signed value between -32768 and 32767. It provides a byte address offset from the beginning of the current array (pointed to by SRC address) to the beginning of the first source array in the next frame. It applies to both A-sync and AB-sync transfers. Note that when SCIDX is applied the current array in an A-sync transfer is the last array in the frame while the current array in a AB-sync transfer is the first array in the frame."]
pub type ScidxW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DCIDX` reader - 31:16\\]
Destination Frame Index: DCIDX is a 16-bit signed value (2's complement) used for destination address modification for the 3rd dimension. It is a signed value between -32768 and 32767. It provides a byte address offset from the beginning of the current array (pointed to by DST address) to the beginning of the first destination array in the next frame. It applies to both A-sync and AB-sync transfers. Note that when DCIDX is applied the current array in an A-sync transfer is the last array in the frame while the current array in a ABsync transfer is the first array in the frame."]
pub type DcidxR = crate::FieldReader<u16>;
#[doc = "Field `DCIDX` writer - 31:16\\]
Destination Frame Index: DCIDX is a 16-bit signed value (2's complement) used for destination address modification for the 3rd dimension. It is a signed value between -32768 and 32767. It provides a byte address offset from the beginning of the current array (pointed to by DST address) to the beginning of the first destination array in the next frame. It applies to both A-sync and AB-sync transfers. Note that when DCIDX is applied the current array in an A-sync transfer is the last array in the frame while the current array in a ABsync transfer is the first array in the frame."]
pub type DcidxW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Source Frame Index: SCIDX is a 16-bit signed value (2's complement) used for source address modification for the 3rd dimension. It is a signed value between -32768 and 32767. It provides a byte address offset from the beginning of the current array (pointed to by SRC address) to the beginning of the first source array in the next frame. It applies to both A-sync and AB-sync transfers. Note that when SCIDX is applied the current array in an A-sync transfer is the last array in the frame while the current array in a AB-sync transfer is the first array in the frame."]
    #[inline(always)]
    pub fn scidx(&self) -> ScidxR {
        ScidxR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Destination Frame Index: DCIDX is a 16-bit signed value (2's complement) used for destination address modification for the 3rd dimension. It is a signed value between -32768 and 32767. It provides a byte address offset from the beginning of the current array (pointed to by DST address) to the beginning of the first destination array in the next frame. It applies to both A-sync and AB-sync transfers. Note that when DCIDX is applied the current array in an A-sync transfer is the last array in the frame while the current array in a ABsync transfer is the first array in the frame."]
    #[inline(always)]
    pub fn dcidx(&self) -> DcidxR {
        DcidxR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Source Frame Index: SCIDX is a 16-bit signed value (2's complement) used for source address modification for the 3rd dimension. It is a signed value between -32768 and 32767. It provides a byte address offset from the beginning of the current array (pointed to by SRC address) to the beginning of the first source array in the next frame. It applies to both A-sync and AB-sync transfers. Note that when SCIDX is applied the current array in an A-sync transfer is the last array in the frame while the current array in a AB-sync transfer is the first array in the frame."]
    #[inline(always)]
    #[must_use]
    pub fn scidx(&mut self) -> ScidxW<CidxSpec> {
        ScidxW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Destination Frame Index: DCIDX is a 16-bit signed value (2's complement) used for destination address modification for the 3rd dimension. It is a signed value between -32768 and 32767. It provides a byte address offset from the beginning of the current array (pointed to by DST address) to the beginning of the first destination array in the next frame. It applies to both A-sync and AB-sync transfers. Note that when DCIDX is applied the current array in an A-sync transfer is the last array in the frame while the current array in a ABsync transfer is the first array in the frame."]
    #[inline(always)]
    #[must_use]
    pub fn dcidx(&mut self) -> DcidxW<CidxSpec> {
        DcidxW::new(self, 16)
    }
}
#[doc = "Register description is not available\n\nYou can [`read`](crate::Reg::read) this register and get [`cidx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cidx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CidxSpec;
impl crate::RegisterSpec for CidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidx::R`](R) reader structure"]
impl crate::Readable for CidxSpec {}
#[doc = "`write(|w| ..)` method takes [`cidx::W`](W) writer structure"]
impl crate::Writable for CidxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CIDX to value 0"]
impl crate::Resettable for CidxSpec {
    const RESET_VALUE: u32 = 0;
}
