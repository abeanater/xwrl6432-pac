#[doc = "Register `MCRC_RESERVED` reader"]
pub type R = crate::R<McrcReservedSpec>;
#[doc = "Register `MCRC_RESERVED` writer"]
pub type W = crate::W<McrcReservedSpec>;
#[doc = "Field `NU68` reader - 31:0\\]
0x144 to 0x1FF is reserved area."]
pub type Nu68R = crate::FieldReader<u32>;
#[doc = "Field `NU68` writer - 31:0\\]
0x144 to 0x1FF is reserved area."]
pub type Nu68W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
0x144 to 0x1FF is reserved area."]
    #[inline(always)]
    pub fn nu68(&self) -> Nu68R {
        Nu68R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
0x144 to 0x1FF is reserved area."]
    #[inline(always)]
    #[must_use]
    pub fn nu68(&mut self) -> Nu68W<McrcReservedSpec> {
        Nu68W::new(self, 0)
    }
}
#[doc = "0x144 to 0x1FF is reserved area.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcrc_reserved::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcrc_reserved::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McrcReservedSpec;
impl crate::RegisterSpec for McrcReservedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcrc_reserved::R`](R) reader structure"]
impl crate::Readable for McrcReservedSpec {}
#[doc = "`write(|w| ..)` method takes [`mcrc_reserved::W`](W) writer structure"]
impl crate::Writable for McrcReservedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCRC_RESERVED to value 0"]
impl crate::Resettable for McrcReservedSpec {
    const RESET_VALUE: u32 = 0;
}
