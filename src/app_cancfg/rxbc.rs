#[doc = "Register `RXBC` reader"]
pub type R = crate::R<RxbcSpec>;
#[doc = "Register `RXBC` writer"]
pub type W = crate::W<RxbcSpec>;
#[doc = "Field `NU48` reader - 1:0\\]
Reserved"]
pub type Nu48R = crate::FieldReader;
#[doc = "Field `NU48` writer - 1:0\\]
Reserved"]
pub type Nu48W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RBSA` reader - 15:2\\]
Rx Buffer Start Address"]
pub type RbsaR = crate::FieldReader<u16>;
#[doc = "Field `RBSA` writer - 15:2\\]
Rx Buffer Start Address"]
pub type RbsaW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `NU49` reader - 31:16\\]
Reserved"]
pub type Nu49R = crate::FieldReader<u16>;
#[doc = "Field `NU49` writer - 31:16\\]
Reserved"]
pub type Nu49W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu48(&self) -> Nu48R {
        Nu48R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:15 - 15:2\\]
Rx Buffer Start Address"]
    #[inline(always)]
    pub fn rbsa(&self) -> RbsaR {
        RbsaR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    pub fn nu49(&self) -> Nu49R {
        Nu49R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu48(&mut self) -> Nu48W<RxbcSpec> {
        Nu48W::new(self, 0)
    }
    #[doc = "Bits 2:15 - 15:2\\]
Rx Buffer Start Address"]
    #[inline(always)]
    #[must_use]
    pub fn rbsa(&mut self) -> RbsaW<RxbcSpec> {
        RbsaW::new(self, 2)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu49(&mut self) -> Nu49W<RxbcSpec> {
        Nu49W::new(self, 16)
    }
}
#[doc = "RXBC\n\nYou can [`read`](crate::Reg::read) this register and get [`rxbc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxbc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxbcSpec;
impl crate::RegisterSpec for RxbcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxbc::R`](R) reader structure"]
impl crate::Readable for RxbcSpec {}
#[doc = "`write(|w| ..)` method takes [`rxbc::W`](W) writer structure"]
impl crate::Writable for RxbcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXBC to value 0"]
impl crate::Resettable for RxbcSpec {
    const RESET_VALUE: u32 = 0;
}
