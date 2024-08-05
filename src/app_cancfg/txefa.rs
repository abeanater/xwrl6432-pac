#[doc = "Register `TXEFA` reader"]
pub type R = crate::R<TxefaSpec>;
#[doc = "Register `TXEFA` writer"]
pub type W = crate::W<TxefaSpec>;
#[doc = "Field `EFAI` reader - 4:0\\]
Event FIFO Acknowledge Index"]
pub type EfaiR = crate::FieldReader;
#[doc = "Field `EFAI` writer - 4:0\\]
Event FIFO Acknowledge Index"]
pub type EfaiW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NU73` reader - 31:5\\]
Reserved"]
pub type Nu73R = crate::FieldReader<u32>;
#[doc = "Field `NU73` writer - 31:5\\]
Reserved"]
pub type Nu73W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Event FIFO Acknowledge Index"]
    #[inline(always)]
    pub fn efai(&self) -> EfaiR {
        EfaiR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Reserved"]
    #[inline(always)]
    pub fn nu73(&self) -> Nu73R {
        Nu73R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Event FIFO Acknowledge Index"]
    #[inline(always)]
    #[must_use]
    pub fn efai(&mut self) -> EfaiW<TxefaSpec> {
        EfaiW::new(self, 0)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu73(&mut self) -> Nu73W<TxefaSpec> {
        Nu73W::new(self, 5)
    }
}
#[doc = "TXEFA\n\nYou can [`read`](crate::Reg::read) this register and get [`txefa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txefa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxefaSpec;
impl crate::RegisterSpec for TxefaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txefa::R`](R) reader structure"]
impl crate::Readable for TxefaSpec {}
#[doc = "`write(|w| ..)` method takes [`txefa::W`](W) writer structure"]
impl crate::Writable for TxefaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXEFA to value 0"]
impl crate::Resettable for TxefaSpec {
    const RESET_VALUE: u32 = 0;
}
